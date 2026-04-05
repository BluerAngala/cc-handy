import React from "react";
import { useTranslation } from "react-i18next";
import { SettingsGroup } from "../../ui/SettingsGroup";
import { LazyStreamClose } from "../LazyStreamClose";
import { PostProcessingCollapsible } from "./PostProcessingCollapsible";

export const ExperimentalSettings: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="max-w-3xl w-full mx-auto space-y-6">
      <SettingsGroup title={t("settings.experimental.groups.features")}>
        {/* 在转录之间保持麦克风开启 */}
        <LazyStreamClose descriptionMode="tooltip" grouped={true} />

        {/* 后处理功能 - 折叠面板组件 */}
        <PostProcessingCollapsible />
      </SettingsGroup>
    </div>
  );
};
