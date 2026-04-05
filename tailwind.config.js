/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Legacy compatibility
        text: "var(--color-text)",
        background: "var(--color-background)",
        "logo-primary": "var(--color-logo-primary)",
        "logo-stroke": "var(--color-logo-stroke)",
        "text-stroke": "var(--color-text-stroke)",
        "mid-gray": "var(--color-mid-gray)",

        // New theme system - Backgrounds
        "bg-primary": "var(--color-bg-primary)",
        "bg-secondary": "var(--color-bg-secondary)",
        "bg-tertiary": "var(--color-bg-tertiary)",
        "bg-elevated": "var(--color-bg-elevated)",
        "bg-overlay": "var(--color-bg-overlay)",

        // New theme system - Text
        "text-primary": "var(--color-text-primary)",
        "text-secondary": "var(--color-text-secondary)",
        "text-tertiary": "var(--color-text-tertiary)",
        "text-inverse": "var(--color-text-inverse)",

        // New theme system - Borders
        "border-primary": "var(--color-border-primary)",
        "border-secondary": "var(--color-border-secondary)",
        "border-focus": "var(--color-border-focus)",

        // New theme system - Interactive
        "interactive-primary": "var(--color-interactive-primary)",
        "interactive-primary-hover": "var(--color-interactive-primary-hover)",
        "interactive-primary-active": "var(--color-interactive-primary-active)",
        "interactive-secondary": "var(--color-interactive-secondary)",
        "interactive-secondary-hover": "var(--color-interactive-secondary-hover)",
        "interactive-secondary-active": "var(--color-interactive-secondary-active)",

        // New theme system - Accents
        "accent-purple": "var(--color-accent-purple)",
        "accent-blue": "var(--color-accent-blue)",
        "accent-gradient-start": "var(--color-accent-gradient-start)",
        "accent-gradient-end": "var(--color-accent-gradient-end)",

        // New theme system - Status
        "status-recording": "var(--color-status-recording)",
        "status-processing": "var(--color-status-processing)",
        "status-success": "var(--color-status-success)",
        "status-idle": "var(--color-status-idle)",

        // Primary palette
        primary: {
          50: "var(--color-primary-50)",
          100: "var(--color-primary-100)",
          200: "var(--color-primary-200)",
          300: "var(--color-primary-300)",
          400: "var(--color-primary-400)",
          500: "var(--color-primary-500)",
          600: "var(--color-primary-600)",
          700: "var(--color-primary-700)",
          800: "var(--color-primary-800)",
          900: "var(--color-primary-900)",
        },

        // Secondary palette
        secondary: {
          50: "var(--color-secondary-50)",
          100: "var(--color-secondary-100)",
          200: "var(--color-secondary-200)",
          300: "var(--color-secondary-300)",
          400: "var(--color-secondary-400)",
          500: "var(--color-secondary-500)",
          600: "var(--color-secondary-600)",
          700: "var(--color-secondary-700)",
          800: "var(--color-secondary-800)",
          900: "var(--color-secondary-900)",
        },

        // Semantic colors
        success: "var(--color-success)",
        warning: "var(--color-warning)",
        error: "var(--color-error)",
        info: "var(--color-info)",

        // Gray palette
        gray: {
          50: "var(--color-gray-50)",
          100: "var(--color-gray-100)",
          200: "var(--color-gray-200)",
          300: "var(--color-gray-300)",
          400: "var(--color-gray-400)",
          500: "var(--color-gray-500)",
          600: "var(--color-gray-600)",
          700: "var(--color-gray-700)",
          800: "var(--color-gray-800)",
          900: "var(--color-gray-900)",
        },
      },
      backgroundImage: {
        "gradient-primary": "linear-gradient(135deg, var(--color-accent-gradient-start), var(--color-accent-gradient-end))",
      },
      boxShadow: {
        "focus": "0 0 0 3px color-mix(in srgb, var(--color-border-focus), transparent 80%)",
        "card": "0 4px 6px -1px color-mix(in srgb, var(--color-text-primary), transparent 90%)",
        "card-hover": "0 10px 15px -3px color-mix(in srgb, var(--color-text-primary), transparent 85%)",
      },
      animation: {
        "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        "bounce-slow": "bounce 2s infinite",
      },
    },
  },
  plugins: [],
};
