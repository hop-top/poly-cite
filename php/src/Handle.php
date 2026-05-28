<?php

declare(strict_types=1);

namespace Hop\Cite;

final class HandlerSpec {
    public const LANGUAGE_GO = "go";
    public const LANGUAGE_TS = "ts";
    public const LANGUAGE_PYTHON = "py";
    public const LANGUAGE_RUST = "rs";
    public const LANGUAGE_PHP = "php";

    public function __construct(
        public readonly string $vendor,
        public readonly string $app,
        public readonly string $language,
        public readonly string $scheme,
        public readonly string $appPath,
        public readonly string $instance = "",
        public readonly string $version = "",
        public readonly string $channel = "",
        public readonly string $displayName = "",
    ) {}

    public function handlerId(): string {
        $this->validate();
        $parts = [$this->vendor, $this->app];
        if ($this->instance !== "") {
            $parts[] = $this->instance;
        }
        $parts[] = $this->language;
        $parts[] = $this->scheme;
        return implode(".", $parts);
    }

    public function validate(): void {
        foreach ([
            "vendor" => $this->vendor,
            "app" => $this->app,
            "language" => $this->language,
            "scheme" => $this->scheme,
            "app_path" => $this->appPath,
        ] as $field => $value) {
            if ($value === "") {
                throw new \InvalidArgumentException("generate: {$field} must not be empty");
            }
        }

        if (!in_array($this->language, [self::LANGUAGE_GO, self::LANGUAGE_TS, self::LANGUAGE_PYTHON, self::LANGUAGE_RUST, self::LANGUAGE_PHP], true)) {
            throw new \InvalidArgumentException("generate: unsupported language \"{$this->language}\"");
        }

        foreach ([
            "vendor" => $this->vendor,
            "app" => $this->app,
            "instance" => $this->instance,
            "language" => $this->language,
            "scheme" => $this->scheme,
        ] as $field => $value) {
            if (str_contains($value, "/") || str_contains($value, "\\")) {
                throw new \InvalidArgumentException("generate: {$field} must not contain path separators");
            }
        }
    }

    public function displayName(): string {
        return $this->displayName !== "" ? $this->displayName : $this->handlerId();
    }
}

final class Handle {
    public static function snippet(string $platform, HandlerSpec $spec): string {
        return match ($platform) {
            "macos", "ios" => self::plistSnippet($spec),
            "linux" => self::desktopFile($spec),
            "windows" => self::windowsRegSnippet($spec),
            default => throw new \InvalidArgumentException("generate: unknown platform \"{$platform}\""),
        };
    }

    public static function desktopFile(HandlerSpec $spec): string {
        $id = $spec->handlerId();
        return "[Desktop Entry]\n"
            . "Type=Application\n"
            . "Name={$spec->displayName()}\n"
            . "Exec={$spec->appPath} %u\n"
            . "MimeType=x-scheme-handler/{$spec->scheme};\n"
            . "NoDisplay=true\n"
            . "X-Hop-Handler-ID={$id}\n";
    }

    public static function desktopFilename(HandlerSpec $spec): string {
        return $spec->handlerId() . ".desktop";
    }

    public static function plistSnippet(HandlerSpec $spec): string {
        $id = $spec->handlerId();
        return "<key>CFBundleURLTypes</key>\n"
            . "<array>\n"
            . "\t<dict>\n"
            . "\t\t<key>CFBundleURLName</key>\n"
            . "\t\t<string>{$id}</string>\n"
            . "\t\t<key>CFBundleURLSchemes</key>\n"
            . "\t\t<array>\n"
            . "\t\t\t<string>{$spec->scheme}</string>\n"
            . "\t\t</array>\n"
            . "\t</dict>\n"
            . "</array>";
    }

    public static function patchPlist(string $plist, HandlerSpec $spec): string {
        $snippet = self::plistSnippet($spec);
        $out = str_replace("</dict>\n</plist>", $snippet . "\n</dict>\n</plist>", $plist, $count);
        if ($count > 0) {
            return $out;
        }
        return str_replace("</dict></plist>", $snippet . "\n</dict></plist>", $plist);
    }

    public static function windowsRegSnippet(HandlerSpec $spec): string {
        $id = $spec->handlerId();
        $displayName = $spec->displayName();
        return "Windows Registry Editor Version 5.00\r\n\r\n"
            . "[HKEY_CURRENT_USER\\Software\\Classes\\{$spec->scheme}]\r\n"
            . "@=\"URL:{$displayName} Protocol\"\r\n"
            . "\"URL Protocol\"=\"\"\r\n"
            . "\"FriendlyTypeName\"=\"{$displayName}\"\r\n"
            . "\"HopHandlerID\"=\"{$id}\"\r\n\r\n"
            . "[HKEY_CURRENT_USER\\Software\\Classes\\{$spec->scheme}\\shell\\open\\command]\r\n"
            . "@=\"\\\"{$spec->appPath}\\\" \\\"%1\\\"\"\r\n";
    }
}
