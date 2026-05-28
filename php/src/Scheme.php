<?php

declare(strict_types=1);

namespace Hop\Cite;

final class URI {
    public function __construct(
        public readonly string $scheme,
        public readonly string $namespace,
        public readonly string $id,
        public readonly string $query = "",
        public readonly string $fragment = "",
        public readonly string $original = "",
        public readonly string $action = "",
    ) {}

    public function canonical(): string {
        $out = $this->scheme . "://" . $this->namespace . "/" . $this->id;
        if ($this->query !== "") {
            $out .= "?" . $this->query;
        }
        if ($this->fragment !== "") {
            $out .= "#" . $this->fragment;
        }
        return $out;
    }

    public function vanity(): string {
        return $this->original === "" ? $this->canonical() : $this->original;
    }

    public function __toString(): string {
        return $this->canonical();
    }
}

final class Policy {
    /** @param array<string,int> $schemeNamespaceSegments @param list<VanityAlias> $vanityAliases @param array<string,ActionRoute> $actionRoutes */
    public function __construct(
        public int $defaultNamespaceSegments = 1,
        public array $schemeNamespaceSegments = [],
        public array $vanityAliases = [],
        public array $actionRoutes = [],
    ) {}

    public function namespaceSegments(string $scheme): int {
        return $this->schemeNamespaceSegments[$scheme] ?? ($this->defaultNamespaceSegments !== 0 ? $this->defaultNamespaceSegments : 1);
    }

    public function resolveAction(URI $uri): ResolvedAction {
        if ($uri->action === "") {
            throw new \InvalidArgumentException("cite: action is required");
        }
        if (!array_key_exists($uri->action, $this->actionRoutes)) {
            throw new \InvalidArgumentException("cite: unknown action \"{$uri->action}\"");
        }
        $route = $this->actionRoutes[$uri->action];
        if (!$route instanceof ActionRoute) {
            throw new \InvalidArgumentException("cite: invalid action route");
        }
        if ($route->command === "") {
            throw new \InvalidArgumentException("cite: action route command is required");
        }

        return new ResolvedAction(
            $uri->action,
            self::expandActionTemplate($route->command, $uri),
            array_map(fn(string $arg): string => self::expandActionTemplate($arg, $uri), $route->args),
        );
    }

    /** @return list<VanityCandidate> */
    public function vanityCandidates(string $input): array {
        $candidates = [];
        foreach ($this->vanityAliases as $alias) {
            if (!$alias instanceof VanityAlias) {
                continue;
            }
            $distance = levenshtein($input, $alias->from);
            if (!self::withinFuzzyThreshold($input, $alias->from, $distance)) {
                continue;
            }
            $candidates[] = new VanityCandidate($alias->from, $alias->to, $distance);
        }
        usort($candidates, static function (VanityCandidate $a, VanityCandidate $b): int {
            if ($a->distance !== $b->distance) {
                return $a->distance <=> $b->distance;
            }
            return $a->from <=> $b->from;
        });
        return $candidates;
    }

    private static function expandActionTemplate(string $value, URI $uri): string {
        return strtr($value, [
            "{scheme}" => $uri->scheme,
            "{namespace}" => $uri->namespace,
            "{id}" => $uri->id,
            "{query}" => $uri->query,
            "{fragment}" => $uri->fragment,
        ]);
    }

    private static function withinFuzzyThreshold(string $input, string $candidate, int $distance): bool {
        $longest = max(strlen($input), strlen($candidate));
        $threshold = intdiv($longest, 5);
        if ($threshold < 2) {
            $threshold = 2;
        }
        if ($threshold > 8) {
            $threshold = 8;
        }
        return $distance <= $threshold;
    }
}

final class ParseOptions {
    public function __construct(
        public bool $strict = false,
        public bool $jsonAmbiguity = false,
    ) {}
}

final class VanityAlias {
    public function __construct(
        public readonly string $from,
        public readonly string $to,
        public readonly bool $prefix = false,
        public readonly bool $preserveSuffix = false,
    ) {}
}

final class VanityCandidate implements \JsonSerializable {
    public function __construct(
        public readonly string $from,
        public readonly string $to,
        public readonly int $distance,
    ) {}

    /** @return array{from:string,to:string,distance:int} */
    public function jsonSerialize(): array {
        return ["from" => $this->from, "to" => $this->to, "distance" => $this->distance];
    }
}

final class AmbiguousVanityException extends \RuntimeException implements \JsonSerializable {
    /** @param list<VanityCandidate> $candidates */
    public function __construct(
        public readonly string $input,
        public readonly array $candidates,
        bool $asJson = false,
    ) {
        parent::__construct($asJson ? json_encode($this, JSON_THROW_ON_ERROR) : self::textMessage($input, $candidates));
    }

    /** @return array{input:string,candidates:list<VanityCandidate>} */
    public function jsonSerialize(): array {
        return ["input" => $this->input, "candidates" => $this->candidates];
    }

    /** @param list<VanityCandidate> $candidates */
    private static function textMessage(string $input, array $candidates): string {
        return "cite: ambiguous vanity alias \"{$input}\": " . implode(", ", array_map(fn(VanityCandidate $c): string => $c->from, $candidates));
    }
}

final class ActionRoute {
    /** @param list<string> $args */
    public function __construct(
        public readonly string $command,
        public readonly array $args = [],
    ) {}
}

final class ResolvedAction {
    /** @param list<string> $args */
    public function __construct(
        public readonly string $action,
        public readonly string $command,
        public readonly array $args = [],
    ) {}
}

final class Scheme {
    public static function defaultPolicy(): Policy {
        return new Policy(1, [
            "task" => 2,
            "doc" => 2,
            "repo" => 1,
            "tlc" => 2,
            "task-dev" => 2,
            "task-stress" => 2,
        ]);
    }

    public static function parse(string $input, ?Policy $policy = null, ?ParseOptions $options = null): URI {
        if ($input === "") {
            throw new \InvalidArgumentException("cite: empty input");
        }
        $policy ??= self::defaultPolicy();
        $options ??= new ParseOptions();
        [$parseInput, $vanity] = self::resolveVanity($input, $policy, $options);

        $parts = parse_url($parseInput);
        if ($parts === false) {
            throw new \InvalidArgumentException("cite: invalid input");
        }
        $scheme = (string)($parts["scheme"] ?? "");
        $host = (string)($parts["host"] ?? "");
        if ($scheme === "") {
            throw new \InvalidArgumentException("cite: scheme is required");
        }
        if ($host === "") {
            throw new \InvalidArgumentException("cite: namespace is required");
        }

        $segments = [$host];
        $path = ltrim((string)($parts["path"] ?? ""), "/");
        if ($path !== "") {
            foreach (explode("/", $path) as $segment) {
                if ($segment !== "") {
                    $segments[] = rawurldecode($segment);
                }
            }
        }

        $namespaceSegments = $policy->namespaceSegments($scheme);
        if ($namespaceSegments <= 0) {
            throw new \InvalidArgumentException("cite: namespace segment count must be positive");
        }
        if (count($segments) <= $namespaceSegments) {
            throw new \InvalidArgumentException("cite: id is required");
        }

        $namespace = implode("/", array_slice($segments, 0, $namespaceSegments));
        $id = implode("/", array_slice($segments, $namespaceSegments));
        if ($namespace === "") {
            throw new \InvalidArgumentException("cite: namespace is required");
        }
        if ($id === "") {
            throw new \InvalidArgumentException("cite: id is required");
        }

        $query = (string)($parts["query"] ?? "");
        return new URI(
            $scheme,
            $namespace,
            $id,
            $query,
            (string)($parts["fragment"] ?? ""),
            $vanity,
            self::actionFromQuery($query),
        );
    }

    /** @return array{0:string,1:string} */
    private static function resolveVanity(string $input, Policy $policy, ParseOptions $options): array {
        $best = null;
        $bestLength = -1;
        foreach ($policy->vanityAliases as $alias) {
            if (!$alias instanceof VanityAlias) {
                throw new \InvalidArgumentException("cite: invalid vanity alias");
            }
            if ($alias->from === "" || $alias->to === "") {
                throw new \InvalidArgumentException("cite: vanity alias from and to are required");
            }
            $matched = $input === $alias->from;
            if (!$matched && $alias->prefix) {
                $matched = str_starts_with($input, $alias->from . "/");
            }
            if ($matched && strlen($alias->from) > $bestLength) {
                $best = $alias;
                $bestLength = strlen($alias->from);
            }
        }

        if ($best instanceof VanityAlias) {
            $target = $best->to;
            if ($best->prefix && $best->preserveSuffix && strlen($input) > strlen($best->from)) {
                $target = rtrim($target, "/") . substr($input, strlen($best->from));
            }
            return [$target, $input];
        }

        if (!$options->strict) {
            $candidates = $policy->vanityCandidates($input);
            if ($candidates !== []) {
                $bestDistance = $candidates[0]->distance;
                $bestCandidates = array_values(array_filter($candidates, fn(VanityCandidate $c): bool => $c->distance === $bestDistance));
                if (count($bestCandidates) > 1) {
                    throw new AmbiguousVanityException($input, $bestCandidates, $options->jsonAmbiguity);
                }
                return [$bestCandidates[0]->to, $input];
            }
        }

        return [$input, ""];
    }

    private static function actionFromQuery(string $rawQuery): string {
        $values = self::parseQueryValues($rawQuery);
        $candidates = [];
        $action = $values["action"] ?? "";
        $name = $values["name"] ?? "";
        if ($action !== "" && $name === "") {
            $candidates[] = $action;
        }

        $cmd = $values["cmd"] ?? "";
        $verb = $values["verb"] ?? "";
        if ($cmd !== "" || $verb !== "") {
            if ($cmd === "" || $verb === "") {
                throw new \InvalidArgumentException("cite: cmd and verb must be provided together");
            }
            $candidates[] = $cmd . "." . $verb;
        }

        if ($name !== "") {
            if ($action === "") {
                throw new \InvalidArgumentException("cite: name and action must be provided together");
            }
            $candidates[] = $name . "." . $action;
        }

        if ($candidates === []) {
            return "";
        }
        $first = $candidates[0];
        foreach (array_slice($candidates, 1) as $candidate) {
            if ($candidate !== $first) {
                throw new \InvalidArgumentException("cite: conflicting action query parameters");
            }
        }
        return $first;
    }

    /** @return array<string,string> */
    private static function parseQueryValues(string $rawQuery): array {
        if ($rawQuery === "") {
            return [];
        }
        $values = [];
        foreach (explode("&", $rawQuery) as $pair) {
            if ($pair === "") {
                continue;
            }
            [$key, $value] = array_pad(explode("=", $pair, 2), 2, "");
            $key = urldecode($key);
            if (!array_key_exists($key, $values)) {
                $values[$key] = urldecode($value);
            }
        }
        return $values;
    }
}
