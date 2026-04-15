import React from "react";
import { useThemeStore, THEMES, type Theme } from "@/stores/themeStore";
import { Check, Monitor, Moon, Sun, Palette } from "lucide-react";
import { useTranslation } from "react-i18next";

interface ThemeSelectorProps {
  className?: string;
}

const ThemeIcon: React.FC<{ themeId: Theme; className?: string }> = ({
  themeId,
  className = "w-5 h-5",
}) => {
  switch (themeId) {
    case "light":
      return <Sun className={className} />;
    case "dark":
      return <Moon className={className} />;
    case "midnight":
      return <Moon className={className} />;
    default:
      return <Palette className={className} />;
  }
};

const ThemePreview: React.FC<{ themeId: Theme }> = ({ themeId }) => {
  const getPreviewColors = (theme: Theme) => {
    switch (theme) {
      case "light":
        return { bg: "#ffffff", accent: "#8b5cf6", text: "#0f172a" };
      case "dark":
        return { bg: "#0f172a", accent: "#a78bfa", text: "#f8fafc" };
      case "purple-dream":
        return { bg: "#faf5ff", accent: "#7c3aed", text: "#581c87" };
      case "midnight":
        return { bg: "#020617", accent: "#60a5fa", text: "#f1f5f9" };
      case "sunset":
        return { bg: "#fff7ed", accent: "#ea580c", text: "#7c2d12" };
      case "forest":
        return { bg: "#f0fdf4", accent: "#16a34a", text: "#14532d" };
      default:
        return { bg: "#ffffff", accent: "#8b5cf6", text: "#0f172a" };
    }
  };

  const colors = getPreviewColors(themeId);

  return (
    <div
      className="w-16 h-10 rounded-lg border-2 border-border-primary overflow-hidden flex-shrink-0"
      style={{ backgroundColor: colors.bg }}
    >
      <div className="h-full flex flex-col p-1.5 gap-1">
        <div
          className="w-8 h-2 rounded-full"
          style={{ backgroundColor: colors.accent }}
        />
        <div
          className="w-full h-1.5 rounded-full opacity-30"
          style={{ backgroundColor: colors.text }}
        />
        <div
          className="w-2/3 h-1.5 rounded-full opacity-30"
          style={{ backgroundColor: colors.text }}
        />
      </div>
    </div>
  );
};

export const ThemeSelector: React.FC<ThemeSelectorProps> = ({
  className = "",
}) => {
  const { t } = useTranslation();
  const { theme, isSystemTheme, setTheme, setSystemTheme } = useThemeStore();

  return (
    <div className={`space-y-4 ${className}`}>
      {/* System Theme Toggle */}
      <div className="flex items-center justify-between p-3 rounded-lg bg-bg-secondary border border-border-primary">
        <div className="flex items-center gap-3">
          <Monitor className="w-5 h-5 text-text-secondary" />
          <div>
            <p className="font-medium text-text-primary">
              {t("settings.theme.followSystem.title")}
            </p>
            <p className="text-sm text-text-tertiary">
              {t("settings.theme.followSystem.description")}
            </p>
          </div>
        </div>
        <button
          onClick={() => setSystemTheme(!isSystemTheme)}
          className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            isSystemTheme
              ? "bg-interactive-primary"
              : "bg-bg-tertiary border border-border-primary"
          }`}
        >
          <span
            className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
              isSystemTheme ? "translate-x-6" : "translate-x-1"
            }`}
          />
        </button>
      </div>

      {/* Theme Grid */}
      {!isSystemTheme && (
        <div className="grid grid-cols-1 gap-3">
          {THEMES.map((themeOption) => (
            <button
              key={themeOption.id}
              onClick={() => setTheme(themeOption.id)}
              className={`flex items-center gap-4 p-3 rounded-xl border-2 transition-all text-left ${
                theme === themeOption.id
                  ? "border-interactive-primary bg-bg-secondary"
                  : "border-border-primary hover:border-border-secondary bg-bg-primary"
              }`}
            >
              <ThemePreview themeId={themeOption.id} />

              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <ThemeIcon
                    themeId={themeOption.id}
                    className={`w-4 h-4 ${
                      theme === themeOption.id
                        ? "text-interactive-primary"
                        : "text-text-secondary"
                    }`}
                  />
                  <span
                    className={`font-medium ${
                      theme === themeOption.id
                        ? "text-text-primary"
                        : "text-text-secondary"
                    }`}
                  >
                    {themeOption.name}
                  </span>
                  {theme === themeOption.id && (
                    <Check className="w-4 h-4 text-interactive-primary" />
                  )}
                </div>
                <p className="text-sm text-text-tertiary mt-0.5">
                  {themeOption.description}
                </p>
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

export default ThemeSelector;
