<?php

declare(strict_types=1);

require __DIR__ . "/../../../php/vendor/autoload.php";

use HopTop\Cite\ActionRoute;
use HopTop\Cite\Handle;
use HopTop\Cite\HandlerSpec;
use HopTop\Cite\ParseOptions;
use HopTop\Cite\Policy;
use HopTop\Cite\Scheme;
use HopTop\Cite\VanityAlias;

$fixtureDir = $argv[1] ?? "spec/fixtures";
$citeFixture = json_decode(file_get_contents($fixtureDir . "/cite-contract.json"), true, flags: JSON_THROW_ON_ERROR);
$handlerFixture = json_decode(file_get_contents($fixtureDir . "/handler-contract.json"), true, flags: JSON_THROW_ON_ERROR);
$basePolicy = new Policy((int)$citeFixture["namespacePolicy"]["defaultNamespaceSegments"], array_map("intval", $citeFixture["namespacePolicy"]["schemeNamespaceSegments"]));

$citeOut = function ($got): array {
    return [
        "scheme" => $got->scheme,
        "namespace" => $got->namespace,
        "id" => $got->id,
        "query" => $got->query,
        "fragment" => $got->fragment,
        "original" => $got->original,
        "action" => $got->action,
        "canonical" => $got->canonical(),
        "vanity" => $got->vanity(),
    ];
};
$parseResult = function (string $name, callable $fn) use ($citeOut): array {
    try {
        $got = $fn();
        return ["name" => $name, "ok" => true, "uri" => $citeOut($got)];
    } catch (Throwable) {
        return ["name" => $name, "ok" => false];
    }
};
$invalidParseResult = function (string $name, callable $fn): array {
    try {
        $fn();
        return ["name" => $name, "ok" => false];
    } catch (Throwable) {
        return ["name" => $name, "ok" => true];
    }
};
$handlerSpec = fn(array $spec): HandlerSpec => new HandlerSpec(
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
$handlerResult = function (array $tc) use ($handlerSpec): array {
    try {
        $spec = $handlerSpec($tc["spec"]);
        return [
            "name" => $tc["name"],
            "ok" => true,
            "handlerId" => $spec->handlerId(),
            "desktopFilename" => $tc["platform"] === "linux" ? Handle::desktopFilename($spec) : "",
            "rendered" => Handle::snippet((string)$tc["platform"], $spec),
        ];
    } catch (Throwable) {
        return ["name" => $tc["name"], "ok" => false];
    }
};
$invalidHandlerResult = function (array $tc) use ($handlerSpec): array {
    try {
        $handlerSpec($tc["spec"])->validate();
        return ["name" => $tc["name"], "ok" => false, "handlerId" => "", "desktopFilename" => "", "rendered" => ""];
    } catch (Throwable) {
        return ["name" => $tc["name"], "ok" => true, "handlerId" => "", "desktopFilename" => "", "rendered" => ""];
    }
};

$output = [
    "version" => 1,
    "uri" => ["valid" => [], "invalid" => [], "vanity" => [], "action" => []],
    "handler" => ["valid" => [], "invalid" => []],
];
foreach ($citeFixture["validCases"] as $tc) {
    $output["uri"]["valid"][] = $parseResult((string)$tc["name"], fn() => Scheme::parse((string)$tc["input"], $basePolicy));
}
foreach ($citeFixture["invalidCases"] as $tc) {
    $output["uri"]["invalid"][] = $invalidParseResult((string)$tc["name"], fn() => Scheme::parse((string)$tc["input"], $basePolicy));
}
foreach ($citeFixture["vanityCases"] as $tc) {
    $aliases = array_map(fn(array $alias): VanityAlias => new VanityAlias((string)$alias["from"], (string)$alias["to"], (bool)($alias["prefix"] ?? false), (bool)($alias["preserveSuffix"] ?? false)), $tc["aliases"] ?? []);
    $policy = new Policy($basePolicy->defaultNamespaceSegments, $basePolicy->schemeNamespaceSegments, $aliases);
    $options = new ParseOptions((bool)($tc["options"]["strict"] ?? false), (bool)($tc["options"]["jsonAmbiguity"] ?? false));
    $output["uri"]["vanity"][] = $parseResult((string)$tc["name"], fn() => Scheme::parse((string)$tc["input"], $policy, $options));
}
foreach ($citeFixture["actionCases"] as $tc) {
    $routes = [];
    foreach ($tc["actionRoutes"] as $name => $route) {
        $routes[$name] = new ActionRoute((string)$route["command"], $route["args"]);
    }
    $policy = new Policy($basePolicy->defaultNamespaceSegments, $basePolicy->schemeNamespaceSegments, actionRoutes: $routes);
    $result = $parseResult((string)$tc["name"], fn() => Scheme::parse((string)$tc["input"], $policy));
    if ($result["ok"]) {
        $plan = $policy->resolveAction(Scheme::parse((string)$tc["input"], $policy));
        $result["action"] = ["action" => $plan->action, "command" => $plan->command, "args" => $plan->args];
    }
    $output["uri"]["action"][] = $result;
}
foreach ($handlerFixture["cases"] as $tc) {
    $output["handler"]["valid"][] = $handlerResult($tc);
}
foreach ($handlerFixture["invalidCases"] as $tc) {
    $output["handler"]["invalid"][] = $invalidHandlerResult($tc);
}

echo json_encode($output, JSON_UNESCAPED_SLASHES) . "\n";
