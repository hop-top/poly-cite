<?php

declare(strict_types=1);

namespace HopTop\Cite;

final class CompletionResult {
    /** @param list<string> $suggestions */
    public function __construct(public readonly array $suggestions) {}
}

final class Completions {
    public static function completeWithScheme(Registry $reg, string $typeName, string $toComplete): CompletionResult {
        $prefix = $toComplete;
        $scheme = "";

        $pos = strpos($toComplete, "://");
        if ($pos !== false) {
            $scheme = substr($toComplete, 0, $pos);
            $prefix = substr($toComplete, $pos + 3);
        }

        if ($scheme !== "") {
            $candidates = $reg->completeVanity($toComplete);
            if (count($candidates) > 1) {
                return new CompletionResult(array_map(
                    fn(VanityCandidate $candidate): string => $candidate->from . "\tcanonical: " . $candidate->to,
                    $candidates,
                ));
            }
        }

        $suggestions = $reg->complete($typeName, $prefix);
        if ($scheme === "") {
            return new CompletionResult($suggestions);
        }
        return new CompletionResult(array_map(fn(string $s): string => $scheme . "://" . $s, $suggestions));
    }
}
