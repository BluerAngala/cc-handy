import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

export type Theme =
  | "light"
  | "dark"
  | "purple-dream"
  | "midnight"
  | "sunset"
  | "forest";

export const THEMES: { id: Theme; name: string; description: string }[] = [
  { id: "light", name: "明亮", description: "清爽明亮的默认主题" },
  { id: "dark", name: "暗黑", description: "护眼的深色主题" },
  { id: "purple-dream", name: "梦幻紫", description: "粉紫色的梦幻风格" },
  { id: "midnight", name: "午夜蓝", description: "深邃的蓝色调" },
  { id: "sunset", name: "日落", description: "温暖的橙黄色调" },
  { id: "forest", name: "森林", description: "清新的绿色调" },
];

interface ThemeStore {
  theme: Theme;
  isSystemTheme: boolean;

  // Actions
  setTheme: (theme: Theme) => void;
  setSystemTheme: (enabled: boolean) => void;
  initialize: () => void;
}

const STORAGE_KEY = "handy-theme";
const STORAGE_SYSTEM_KEY = "handy-system-theme";

const getStoredTheme = (): Theme => {
  if (typeof window === "undefined") return "light";
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored && THEMES.some((t) => t.id === stored)) {
    return stored as Theme;
  }
  return "light";
};

const getStoredSystemTheme = (): boolean => {
  if (typeof window === "undefined") return false;
  const stored = localStorage.getItem(STORAGE_SYSTEM_KEY);
  return stored === "true";
};

const applyTheme = (theme: Theme) => {
  if (typeof document === "undefined") return;
  document.documentElement.setAttribute("data-theme", theme);
};

const applySystemTheme = (enabled: boolean) => {
  if (typeof document === "undefined") return;
  if (enabled) {
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;
    const theme: Theme = prefersDark ? "dark" : "light";
    applyTheme(theme);
  }
};

export const useThemeStore = create<ThemeStore>()(
  subscribeWithSelector((set, get) => ({
    theme: getStoredTheme(),
    isSystemTheme: getStoredSystemTheme(),

    setTheme: (theme: Theme) => {
      set({ theme, isSystemTheme: false });
      localStorage.setItem(STORAGE_KEY, theme);
      localStorage.setItem(STORAGE_SYSTEM_KEY, "false");
      applyTheme(theme);
    },

    setSystemTheme: (enabled: boolean) => {
      set({ isSystemTheme: enabled });
      localStorage.setItem(STORAGE_SYSTEM_KEY, enabled.toString());
      if (enabled) {
        applySystemTheme(true);
      } else {
        applyTheme(get().theme);
      }
    },

    initialize: () => {
      const theme = getStoredTheme();
      const isSystem = getStoredSystemTheme();
      set({ theme, isSystemTheme: isSystem });

      if (isSystem) {
        applySystemTheme(true);
        // Listen for system theme changes
        const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        mediaQuery.addEventListener("change", (e) => {
          if (get().isSystemTheme) {
            applyTheme(e.matches ? "dark" : "light");
          }
        });
      } else {
        applyTheme(theme);
      }
    },
  })),
);
