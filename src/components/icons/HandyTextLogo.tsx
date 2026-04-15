import React from "react";
import { useTranslation } from "react-i18next";

const HandyTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  const { t } = useTranslation();
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 400 100"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <text
        x="10"
        y="75"
        fontFamily="-apple-system, sans-serif"
        fontSize="70"
        fontWeight="900"
        fill="currentColor"
      >
        {t("app.name")}
      </text>
    </svg>
  );
};

export default HandyTextLogo;
