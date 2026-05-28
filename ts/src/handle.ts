export type Language = "go" | "ts" | "py" | "rs" | "php";

export type HandlerSpec = {
  vendor: string;
  app: string;
  instance?: string;
  language: Language | string;
  scheme: string;
  version?: string;
  channel?: string;
  appPath: string;
  displayName?: string;
};

const supportedLanguages = new Set(["go", "ts", "py", "rs", "php"]);

export function validateHandlerSpec(spec: HandlerSpec): void {
  const required: Record<string, string | undefined> = {
    vendor: spec.vendor,
    app: spec.app,
    language: spec.language,
    scheme: spec.scheme,
    app_path: spec.appPath,
  };

  for (const [field, value] of Object.entries(required)) {
    if (!value) throw new Error(`generate: ${field} must not be empty`);
  }

  if (!supportedLanguages.has(spec.language)) throw new Error(`generate: unsupported language ${JSON.stringify(spec.language)}`);

  const identityFields: Record<string, string | undefined> = {
    vendor: spec.vendor,
    app: spec.app,
    instance: spec.instance,
    language: spec.language,
    scheme: spec.scheme,
  };
  for (const [field, value] of Object.entries(identityFields)) {
    if (value && /[\\/]/.test(value)) throw new Error(`generate: ${field} must not contain path separators`);
  }
}

export function handlerID(spec: HandlerSpec): string {
  validateHandlerSpec(spec);
  const parts = [spec.vendor, spec.app];
  if (spec.instance) parts.push(spec.instance);
  parts.push(spec.language, spec.scheme);
  return parts.join(".");
}

export function desktopFilename(spec: HandlerSpec): string {
  return `${handlerID(spec)}.desktop`;
}

export function snippet(platform: string, spec: HandlerSpec): string {
  switch (platform) {
    case "macos":
    case "ios":
      return plistSnippet(spec);
    case "linux":
      return desktopFile(spec);
    case "windows":
      return windowsRegSnippet(spec);
    default:
      throw new Error(`generate: unknown platform ${JSON.stringify(platform)}`);
  }
}

export function desktopFile(spec: HandlerSpec): string {
  const id = handlerID(spec);
  return `[Desktop Entry]\nType=Application\nName=${displayName(spec)}\nExec=${spec.appPath} %u\nMimeType=x-scheme-handler/${spec.scheme};\nNoDisplay=true\nX-Hop-Handler-ID=${id}\n`;
}

export function plistSnippet(spec: HandlerSpec): string {
  const id = handlerID(spec);
  return `<key>CFBundleURLTypes</key>\n<array>\n\t<dict>\n\t\t<key>CFBundleURLName</key>\n\t\t<string>${id}</string>\n\t\t<key>CFBundleURLSchemes</key>\n\t\t<array>\n\t\t\t<string>${spec.scheme}</string>\n\t\t</array>\n\t</dict>\n</array>`;
}

export function windowsRegSnippet(spec: HandlerSpec): string {
  const id = handlerID(spec);
  const name = displayName(spec);
  return `Windows Registry Editor Version 5.00\r\n\r\n[HKEY_CURRENT_USER\\Software\\Classes\\${spec.scheme}]\r\n@="URL:${name} Protocol"\r\n"URL Protocol"=""\r\n"FriendlyTypeName"="${name}"\r\n"HopHandlerID"="${id}"\r\n\r\n[HKEY_CURRENT_USER\\Software\\Classes\\${spec.scheme}\\shell\\open\\command]\r\n@="\\"${spec.appPath}\\" \\"%1\\""\r\n`;
}

function displayName(spec: HandlerSpec): string {
  return spec.displayName || handlerID(spec) || spec.app;
}
