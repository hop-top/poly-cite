<?php

declare(strict_types=1);

namespace Hop\Cite\Tests;

use Hop\Cite\ActionRoute;
use Hop\Cite\AmbiguousVanityException;
use Hop\Cite\Completions;
use Hop\Cite\Handle;
use Hop\Cite\HandlerSpec;
use Hop\Cite\Policy;
use Hop\Cite\ParseOptions;
use Hop\Cite\Registry;
use Hop\Cite\TypeRegistration;
use Hop\Cite\Scheme;
use Hop\Cite\VanityAlias;
use PHPUnit\Framework\TestCase;

final class ContractTest extends TestCase {
    /** @return array<string,mixed> */
    private static function loadUriFixture(): array {
        $raw = file_get_contents(__DIR__ . "/../../spec/fixtures/cite-contract.json");
        self::assertIsString($raw);
        $decoded = json_decode($raw, true, flags: JSON_THROW_ON_ERROR);
        self::assertIsArray($decoded);
        return $decoded;
    }

    /** @return array<string,mixed> */
    private static function loadHandlerFixture(): array {
        $raw = file_get_contents(__DIR__ . "/../../spec/fixtures/handler-contract.json");
        self::assertIsString($raw);
        $decoded = json_decode($raw, true, flags: JSON_THROW_ON_ERROR);
        self::assertIsArray($decoded);
        return $decoded;
    }

    /** @param array<string,mixed> $fixture */
    private static function policyFromFixture(array $fixture): Policy {
        $policy = $fixture["namespacePolicy"];
        return new Policy(
            (int)$policy["defaultNamespaceSegments"],
            array_map("intval", $policy["schemeNamespaceSegments"]),
        );
    }

    /** @param array<string,mixed> $spec */
    private static function handlerSpecFromFixture(array $spec): HandlerSpec {
        return new HandlerSpec(
            (string)($spec["vendor"] ?? ""),
            (string)($spec["app"] ?? ""),
            (string)($spec["language"] ?? ""),
            (string)($spec["scheme"] ?? ""),
            (string)($spec["appPath"] ?? ""),
            (string)($spec["instance"] ?? ""),
            (string)($spec["version"] ?? ""),
            (string)($spec["channel"] ?? ""),
            (string)($spec["displayName"] ?? ""),
        );
    }

    public function testUriValidContractCases(): void {
        $fixture = self::loadUriFixture();
        $policy = self::policyFromFixture($fixture);

        foreach ($fixture["validCases"] as $case) {
            $got = Scheme::parse((string)$case["input"], $policy);
            $expected = $case["expected"];

            $this->assertSame((string)$expected["scheme"], $got->scheme, (string)$case["name"]);
            $this->assertSame((string)$expected["namespace"], $got->namespace, (string)$case["name"]);
            $this->assertSame((string)$expected["id"], $got->id, (string)$case["name"]);
            $this->assertSame((string)($expected["query"] ?? ""), $got->query, (string)$case["name"]);
            $this->assertSame((string)($expected["fragment"] ?? ""), $got->fragment, (string)$case["name"]);
            $this->assertSame((string)($expected["original"] ?? ""), $got->original, (string)$case["name"]);
            $this->assertSame((string)($expected["action"] ?? ""), $got->action, (string)$case["name"]);
            $this->assertSame((string)$case["canonical"], $got->canonical(), (string)$case["name"]);
            $this->assertSame((string)$case["canonical"], (string)$got, (string)$case["name"]);
        }
    }

    public function testUriInvalidContractCases(): void {
        $fixture = self::loadUriFixture();
        $policy = self::policyFromFixture($fixture);

        foreach ($fixture["invalidCases"] as $case) {
            try {
                Scheme::parse((string)$case["input"], $policy);
                $this->fail("expected invalid URI: " . (string)$case["name"]);
            } catch (\InvalidArgumentException) {
                $this->addToAssertionCount(1);
            }
        }
    }

    public function testVanityContractCases(): void {
        $fixture = self::loadUriFixture();
        $basePolicy = self::policyFromFixture($fixture);

        foreach ($fixture["vanityCases"] as $case) {
            $aliases = array_map(
                fn(array $alias): VanityAlias => new VanityAlias(
                    (string)$alias["from"],
                    (string)$alias["to"],
                    (bool)($alias["prefix"] ?? false),
                    (bool)($alias["preserveSuffix"] ?? false),
                ),
                $case["aliases"],
            );
            $policy = new Policy(
                $basePolicy->defaultNamespaceSegments,
                $basePolicy->schemeNamespaceSegments,
                $aliases,
            );
            $options = new ParseOptions((bool)($case["options"]["strict"] ?? false), (bool)($case["options"]["jsonAmbiguity"] ?? false));

            try {
                $got = Scheme::parse((string)$case["input"], $policy, $options);
            } catch (\Throwable $e) {
                if (!(bool)$case["valid"]) {
                    $this->addToAssertionCount(1);
                    continue;
                }
                throw $e;
            }

            $this->assertTrue((bool)$case["valid"], (string)$case["name"]);
            $expected = $case["expected"];
            $this->assertSame((string)$expected["scheme"], $got->scheme, (string)$case["name"]);
            $this->assertSame((string)$expected["namespace"], $got->namespace, (string)$case["name"]);
            $this->assertSame((string)$expected["id"], $got->id, (string)$case["name"]);
            $this->assertSame((string)($expected["original"] ?? ""), $got->original, (string)$case["name"]);
            $this->assertSame((string)$case["canonical"], $got->canonical(), (string)$case["name"]);
        }
    }

    public function testActionContractCases(): void {
        $fixture = self::loadUriFixture();
        $basePolicy = self::policyFromFixture($fixture);

        foreach ($fixture["actionCases"] as $case) {
            $routes = [];
            foreach ($case["actionRoutes"] as $name => $route) {
                $routes[$name] = new ActionRoute((string)$route["command"], $route["args"]);
            }
            $policy = new Policy(
                $basePolicy->defaultNamespaceSegments,
                $basePolicy->schemeNamespaceSegments,
                actionRoutes: $routes,
            );

            $got = Scheme::parse((string)$case["input"], $policy);
            $this->assertSame((string)$case["expected"]["action"], $got->action, (string)$case["name"]);
            $this->assertSame((string)$case["canonical"], $got->canonical(), (string)$case["name"]);

            $resolved = $policy->resolveAction($got);
            $this->assertSame((string)$case["command"], $resolved->command, (string)$case["name"]);
            $this->assertSame($case["args"], $resolved->args, (string)$case["name"]);
        }
    }

    public function testAmbiguousFuzzyVanityCanReturnJsonMessage(): void {
        $policy = new Policy(1, ["task" => 2], [
            new VanityAlias("task://shortcuta", "task://hop-top/cite/T-0001"),
            new VanityAlias("task://shortcutb", "task://hop-top/cite/T-0002"),
        ]);

        try {
            Scheme::parse("task://shortcut", $policy, new ParseOptions(jsonAmbiguity: true));
            $this->fail("expected ambiguous vanity exception");
        } catch (AmbiguousVanityException $e) {
            $this->assertCount(2, $e->candidates);
            $this->assertJsonStringEqualsJsonString('{"input":"task://shortcut","candidates":[{"from":"task://shortcuta","to":"task://hop-top/cite/T-0001","distance":1},{"from":"task://shortcutb","to":"task://hop-top/cite/T-0002","distance":1}]}', $e->getMessage());
        }
    }

    public function testConflictingActionQueryParametersFail(): void {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage("conflicting action query parameters");
        Scheme::parse("tlc://org/repo/T-0001?action=task.claim&cmd=task&verb=close");
    }

    public function testRegistryParseCompleteAndTypesParity(): void {
        $registry = Registry::newRegistry();
        $registry->register(new TypeRegistration("task", function (string $prefix): array {
            return array_values(array_filter(["hop-top/cite/T-0001", "hop-top/cite/T-0002"], fn(string $id): bool => str_starts_with($id, $prefix)));
        }));
        $registry->register(new TypeRegistration("repo"));

        $this->assertSame(["repo", "task"], $registry->types());
        $this->assertSame("T-0001", $registry->parse("task://hop-top/cite/T-0001")->id);
        $this->assertSame(["hop-top/cite/T-0001", "hop-top/cite/T-0002"], $registry->complete("task", "hop-top/cite/T-000"));
        $this->assertSame([], $registry->complete("repo", ""));
    }

    public function testCompletionsPreserveSchemeAndSurfaceVanityCandidates(): void {
        $policy = new Policy(1, ["task" => 2], [
            new VanityAlias("task://shortcuta", "task://hop-top/cite/T-0001"),
            new VanityAlias("task://shortcutb", "task://hop-top/cite/T-0002"),
        ]);
        $registry = Registry::newRegistry($policy);
        $registry->register(new TypeRegistration("task", fn(string $_prefix): array => ["hop-top/cite/T-0001"]));

        $this->assertSame(["task://hop-top/cite/T-0001"], Completions::completeWithScheme($registry, "task", "task://hop-top/cite/T-")->suggestions);
        $this->assertSame([
            "task://shortcuta\tcanonical: task://hop-top/cite/T-0001",
            "task://shortcutb\tcanonical: task://hop-top/cite/T-0002",
        ], Completions::completeWithScheme($registry, "task", "task://shortcut")->suggestions);
    }

    public function testHandlerContractCases(): void {
        $fixture = self::loadHandlerFixture();

        foreach ($fixture["cases"] as $case) {
            $spec = self::handlerSpecFromFixture($case["spec"]);
            $this->assertSame((string)$case["expected"]["handlerId"], $spec->handlerId(), (string)$case["name"]);
            if (isset($case["expected"]["desktopFilename"])) {
                $this->assertSame((string)$case["expected"]["desktopFilename"], Handle::desktopFilename($spec), (string)$case["name"]);
            }
            $rendered = Handle::snippet((string)$case["platform"], $spec);
            foreach ($case["expected"]["renderedContains"] as $expected) {
                $this->assertStringContainsString((string)$expected, $rendered, (string)$case["name"]);
            }
        }
    }

    public function testHandlerInvalidContractCases(): void {
        $fixture = self::loadHandlerFixture();

        foreach ($fixture["invalidCases"] as $case) {
            try {
                self::handlerSpecFromFixture($case["spec"])->validate();
                $this->fail("expected invalid handler: " . (string)$case["name"]);
            } catch (\InvalidArgumentException) {
                $this->addToAssertionCount(1);
            }
        }
    }
}
