#include <CLI/CLI.hpp>

#include <tgbot/net/CurlHttpClient.h>
#include <tgbot/tgbot.h>

#include <spdlog/logger.h>
#include <spdlog/sinks/daily_file_sink.h>

#include <iostream>
#include <optional>
#include <regex>
#include <string>
#include <vector>

bool IsPossiblyFormatted(const std::vector<TgBot::MessageEntity::Ptr>& entities)
{
    // Lowering false positive rate just trust user if he tried to use proper tags in his messages
    for (const auto& entity: entities)
    {
        if (entity->type == "code" || entity->type == "pre")
        {
            return true;
        }
    }

    return false;
}

bool DetectCode(const std::string& text)
{
    static const std::regex cppCodePatterns("std::|using namespace|int main|void main|cout|cin|printf|scanf|#include");
    return std::regex_search(text, cppCodePatterns);
}

int main(int argc, char* argv[])
{
    CLI::App app("CodeDetector Telegram Bot");

    std::string token;
    app.add_option("--token", token, "Telegram Bot API token")->required();

    std::string LogsPath = "logs/log.txt";
    app.add_option("--log-path", LogsPath, "Path to log folder");

    std::optional<std::string> CertificatePath;
    app.add_option("--ca-info", CertificatePath, "Path to a certificate");

    CLI11_PARSE(app, argc, argv);

    TgBot::CurlHttpClient httpClient;
    // Curl configuration
    if(CertificatePath.has_value())
    {
        curl_easy_setopt(httpClient.curlSettings, CURLOPT_CAINFO, CertificatePath.value().c_str());
    }

    auto daily_logger = spdlog::daily_logger_mt("daily_logger", LogsPath, 0, 0);
    daily_logger->flush_on(spdlog::level::info);

    TgBot::Bot bot(token, httpClient);
    bot.getEvents().onAnyMessage([&bot, daily_logger](TgBot::Message::Ptr message)
    {
        if (IsPossiblyFormatted(message->entities))
        {
            return;
        }

        if (DetectCode(message->text))
        {
            bot.getApi().sendMessage(
                message->chat->id,
                "Оберните код в теги: 3 символа ` до и после кода. Спасибо!",
                false,
                message->messageId);
        }
    });

    bot.getEvents().onCommand("help", [&bot, daily_logger](TgBot::Message::Ptr message)
    {
        daily_logger->info("Help command requested");

        bot.getApi().sendMessage(message->chat->id, "The bot reminds you about proper code wrapping. "
            "Unfortunately there are some false positives and false negatives. "
            "If you found any - please report on https://github.com/ZaMaZaN4iK/codedetection-telegram");
    });

    try
    {
        std::cout << "Bot username: " << bot.getApi().getMe()->username << std::endl;
        TgBot::TgLongPoll longPoll(bot, 100, 10);
        while (true)
        {
            std::cout << "Long poll started\n";
            longPoll.start();
        }
    }
    catch (const TgBot::TgException& e)
    {
        std::cout << "Telegram bot exception: " << e.what() << std::endl;
    }
    catch (const std::exception& e)
    {
        std::cout << "Exception: " << e.what() << std::endl;
    }
    return 0;
}