<?php

declare(strict_types=1);

namespace Hop\Cite;

final class TypeRegistration {
    /** @var callable(string): URI|null */
    public $parser;

    /** @var callable(string): list<string>|null */
    public $completer;

    public function __construct(
        public readonly string $name,
        ?callable $completer = null,
        ?callable $parser = null,
    ) {
        $this->completer = $completer;
        $this->parser = $parser;
    }
}

final class Registry {
    /** @var array<string,TypeRegistration> */
    private array $types = [];

    public function __construct(private readonly Policy $policy = new Policy()) {}

    public function register(TypeRegistration $reg): void {
        if ($reg->name === "") {
            throw new \InvalidArgumentException("cite: registration name is required");
        }
        if (array_key_exists($reg->name, $this->types)) {
            throw new \InvalidArgumentException("cite: type \"{$reg->name}\" already registered");
        }
        $this->types[$reg->name] = $reg;
    }

    public function parse(string $input): URI {
        $parsed = Scheme::parse($input, $this->policy);
        if (!array_key_exists($parsed->scheme, $this->types)) {
            throw new \InvalidArgumentException("cite: unknown type \"{$parsed->scheme}\"");
        }
        $reg = $this->types[$parsed->scheme];
        if ($reg->parser !== null) {
            $fn = $reg->parser;
            return $fn($input);
        }
        return $parsed;
    }

    /** @return list<VanityCandidate> */
    public function completeVanity(string $input): array {
        return $this->policy->vanityCandidates($input);
    }

    /** @return list<string> */
    public function complete(string $typeName, string $prefix): array {
        if (!array_key_exists($typeName, $this->types)) {
            throw new \InvalidArgumentException("cite: unknown type \"{$typeName}\"");
        }
        $reg = $this->types[$typeName];
        if ($reg->completer === null) {
            return [];
        }
        $fn = $reg->completer;
        /** @var list<string> $out */
        $out = $fn($prefix);
        return $out;
    }

    /** @return list<string> */
    public function types(): array {
        $types = array_keys($this->types);
        sort($types);
        return array_values($types);
    }

    public static function newRegistry(?Policy $policy = null): Registry {
        return new Registry($policy ?? Scheme::defaultPolicy());
    }
}
