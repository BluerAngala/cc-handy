import React, { useState, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { CollapsiblePanel } from "../../ui/CollapsiblePanel";
import {
  PostProcessingSettingsApi,
  PostProcessingSettingsPrompts,
} from "../post-processing/PostProcessingSettings";
import { ShortcutInput } from "../ShortcutInput";
import { useSettings } from "../../../hooks/useSettings";
import { Tooltip } from "../../ui/Tooltip";

export const PostProcessingCollapsible: React.FC = () => {
  const { t } = useTranslation();
  const { getSetting, updateSetting } = useSettings();
  const [isExpanded, setIsExpanded] = useState(false);
  const [showTooltip, setShowTooltip] = useState(false);
  const tooltipRef = useRef<HTMLDivElement>(null);

  const postProcessEnabled = getSetting("post_process_enabled") || false;

  // 当后处理功能开启时，自动展开折叠面板
  useEffect(() => {
    if (postProcessEnabled) {
      setIsExpanded(true);
    }
  }, [postProcessEnabled]);

  // 处理折叠面板展开/折叠状态变化
  const handleExpandedChange = (expanded: boolean) => {
    setIsExpanded(expanded);
    // 同步更新设置中的开关状态
    if (expanded !== postProcessEnabled) {
      updateSetting("post_process_enabled", expanded);
    }
  };

  const title = (
    <div className="flex items-center gap-2">
      <h3 className="text-sm font-medium">
        {t("settings.debug.postProcessingToggle.label")}
      </h3>
      <div
        ref={tooltipRef}
        className="relative"
        onMouseEnter={() => setShowTooltip(true)}
        onMouseLeave={() => setShowTooltip(false)}
      >
        <svg
          className="w-4 h-4 text-mid-gray cursor-help hover:text-logo-primary transition-colors duration-200 select-none"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        {showTooltip && (
          <Tooltip targetRef={tooltipRef} position="top">
            <p className="text-sm text-center leading-relaxed">
              {t("settings.debug.postProcessingToggle.description")}
            </p>
          </Tooltip>
        )}
      </div>
    </div>
  );

  return (
    <CollapsiblePanel
      expanded={isExpanded}
      onExpandedChange={handleExpandedChange}
      title={title}
    >
      <div className="p-4 space-y-6 bg-background">
        {/* 快捷键设置 */}
        <div>
          <h4 className="text-xs font-medium text-mid-gray uppercase tracking-wide mb-3">
            {t("settings.postProcessing.hotkey.title")}
          </h4>
          <div className="bg-background border border-mid-gray/20 rounded-lg">
            <ShortcutInput
              shortcutId="transcribe_with_post_process"
              descriptionMode="tooltip"
              grouped={true}
            />
          </div>
        </div>

        {/* API 设置 */}
        <div>
          <h4 className="text-xs font-medium text-mid-gray uppercase tracking-wide mb-3">
            {t("settings.postProcessing.api.title")}
          </h4>
          <div className="bg-background border border-mid-gray/20 rounded-lg divide-y divide-mid-gray/20">
            <PostProcessingSettingsApi />
          </div>
        </div>

        {/* Prompts 设置 */}
        <div>
          <h4 className="text-xs font-medium text-mid-gray uppercase tracking-wide mb-3">
            {t("settings.postProcessing.prompts.title")}
          </h4>
          <div className="bg-background border border-mid-gray/20 rounded-lg divide-y divide-mid-gray/20">
            <PostProcessingSettingsPrompts />
          </div>
        </div>
      </div>
    </CollapsiblePanel>
  );
};
