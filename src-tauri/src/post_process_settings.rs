use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::HashMap;

use crate::settings::AppSettings;

/// Voice command patterns for dynamic prompt selection
/// Maps prompt IDs to their voice command keywords
pub fn get_prompt_voice_commands() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    // 基础文字纠错
    map.insert(
        "basic_text_correction".to_string(),
        vec![
            "基础纠错".to_string(),
            "文字纠错".to_string(),
            "基础模式".to_string(),
            "basic correction".to_string(),
        ],
    );

    // 深度整理优化
    map.insert(
        "deep_polish".to_string(),
        vec![
            "深度整理".to_string(),
            "深度优化".to_string(),
            "深度模式".to_string(),
            "deep polish".to_string(),
            "polish".to_string(),
        ],
    );

    // 会议记录整理
    map.insert(
        "meeting_minutes".to_string(),
        vec![
            "会议记录".to_string(),
            "会议纪要".to_string(),
            "会议模式".to_string(),
            "meeting minutes".to_string(),
            "meeting".to_string(),
        ],
    );

    // 思路整理与写作
    map.insert(
        "idea_organization".to_string(),
        vec![
            "思路整理".to_string(),
            "写作辅助".to_string(),
            "写作模式".to_string(),
            "idea organization".to_string(),
            "writing".to_string(),
        ],
    );

    // 关键词标注模式
    map.insert(
        "keyword_highlight".to_string(),
        vec![
            "关键词标注".to_string(),
            "关键词模式".to_string(),
            "标注模式".to_string(),
            "keyword highlight".to_string(),
            "highlight".to_string(),
        ],
    );

    // 访谈采访整理
    map.insert(
        "interview_transcript".to_string(),
        vec![
            "访谈整理".to_string(),
            "采访记录".to_string(),
            "访谈模式".to_string(),
            "interview".to_string(),
        ],
    );

    // 学术技术文档
    map.insert(
        "academic_document".to_string(),
        vec![
            "学术文档".to_string(),
            "技术文档".to_string(),
            "学术模式".to_string(),
            "academic".to_string(),
            "technical".to_string(),
        ],
    );

    // 简洁修正模式
    map.insert(
        "minimal_correction".to_string(),
        vec![
            "简洁修正".to_string(),
            "最小修正".to_string(),
            "简洁模式".to_string(),
            "minimal correction".to_string(),
            "minimal".to_string(),
        ],
    );

    map
}

/// Detect voice command in transcription and return (cleaned_text, prompt_id)
/// Supports patterns like:
/// - "使用会议记录模式"
/// - "切换到关键词标注"
/// - "用深度整理模式"
/// - "meeting minutes mode"
pub fn detect_voice_command(transcription: &str) -> (String, Option<String>) {
    let commands = get_prompt_voice_commands();
    let text = transcription.trim();

    // Pattern: "使用/切换到/用 + [command] + [模式/方式]"
    // Pattern: "[command] + [模式/方式]"
    // Pattern: "[command] + mode"

    for (prompt_id, keywords) in &commands {
        for keyword in keywords {
            // Check various patterns
            let patterns = vec![
                format!("使用{}{}", keyword, "模式"),
                format!("使用{}{}", keyword, "方式"),
                format!("切换到{}{}", keyword, "模式"),
                format!("切换到{}{}", keyword, "方式"),
                format!("用{}{}", keyword, "模式"),
                format!("用{}{}", keyword, "方式"),
                format!("{}{}", keyword, "模式"),
                format!("{}{}", keyword, "方式"),
                format!("{}{}", keyword, " mode"),
                format!("use {} mode", keyword),
                format!("switch to {}", keyword),
                keyword.clone(),
            ];

            for pattern in &patterns {
                if text.contains(pattern) {
                    // Remove the command from text
                    let cleaned = text.replace(pattern, "").trim().to_string();
                    // Clean up any leftover punctuation or spaces
                    let cleaned = cleaned
                        .trim_start_matches(|c| {
                            c == '，' || c == ',' || c == '。' || c == '.' || c == ' '
                        })
                        .to_string();
                    return (cleaned, Some(prompt_id.clone()));
                }
            }
        }
    }

    // No command detected
    (text.to_string(), None)
}

pub const APPLE_INTELLIGENCE_PROVIDER_ID: &str = "apple_intelligence";
pub const APPLE_INTELLIGENCE_DEFAULT_MODEL_ID: &str = "Apple Intelligence";

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct LLMPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct PostProcessProvider {
    pub id: String,
    pub label: String,
    pub base_url: String,
    #[serde(default)]
    pub allow_base_url_edit: bool,
    #[serde(default)]
    pub models_endpoint: Option<String>,
    #[serde(default)]
    pub supports_structured_output: bool,
}

pub fn default_post_process_enabled() -> bool {
    false
}

pub fn default_post_process_provider_id() -> String {
    "siliconflow".to_string()
}

// 默认供应商设置
pub fn default_post_process_providers() -> Vec<PostProcessProvider> {
    let mut providers = vec![
        PostProcessProvider {
            id: "siliconflow".to_string(),
            label: "硅基流动".to_string(),
            base_url: "https://api.siliconflow.cn/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "openai".to_string(),
            label: "OpenAI".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "zai".to_string(),
            label: "Z.AI".to_string(),
            base_url: "https://api.z.ai/api/paas/v4".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "openrouter".to_string(),
            label: "OpenRouter".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
        PostProcessProvider {
            id: "anthropic".to_string(),
            label: "Anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: false,
        },
        PostProcessProvider {
            id: "groq".to_string(),
            label: "Groq".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: false,
        },
        PostProcessProvider {
            id: "cerebras".to_string(),
            label: "Cerebras".to_string(),
            base_url: "https://api.cerebras.ai/v1".to_string(),
            allow_base_url_edit: false,
            models_endpoint: Some("/models".to_string()),
            supports_structured_output: true,
        },
    ];

    // Note: We always include Apple Intelligence on macOS ARM64 without checking availability
    // at startup. The availability check is deferred to when the user actually tries to use it
    // (in actions.rs). This prevents crashes on macOS 26.x beta where accessing
    // SystemLanguageModel.default during early app initialization causes SIGABRT.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        providers.push(PostProcessProvider {
            id: APPLE_INTELLIGENCE_PROVIDER_ID.to_string(),
            label: "Apple Intelligence".to_string(),
            base_url: "apple-intelligence://local".to_string(),
            allow_base_url_edit: false,
            models_endpoint: None,
            supports_structured_output: true,
        });
    }

    // Custom provider always comes last
    providers.push(PostProcessProvider {
        id: "custom".to_string(),
        label: "Custom".to_string(),
        base_url: "http://localhost:11434/v1".to_string(),
        allow_base_url_edit: true,
        models_endpoint: Some("/models".to_string()),
        supports_structured_output: false,
    });

    providers
}

pub fn default_post_process_api_keys() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(provider.id, String::new());
    }
    map
}

pub fn default_model_for_provider(provider_id: &str) -> String {
    if provider_id == APPLE_INTELLIGENCE_PROVIDER_ID {
        return APPLE_INTELLIGENCE_DEFAULT_MODEL_ID.to_string();
    }
    String::new()
}

pub fn default_post_process_models() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for provider in default_post_process_providers() {
        map.insert(
            provider.id.clone(),
            default_model_for_provider(&provider.id),
        );
    }
    map
}

pub fn default_post_process_prompts() -> Vec<LLMPrompt> {
    vec![
        // 1. 基础文字纠错 - 针对语音识别常见问题
        LLMPrompt {
            id: "basic_text_correction".to_string(),
            name: "basic_text_correction".to_string(), // i18n key: settings.postProcessing.prompts.builtin.basicTextCorrection
            prompt: "请对以下语音转录文本进行基础纠错处理：\n\n1. **错别字修正**：修正语音识别产生的同音字错误（如\"在见\"→\"再见\"，\"马上\"→\"马上\"）\n2. **重复处理**：删除无意义的重复词语和重复句子（保留有意义的强调重复）\n3. **标点优化**：添加适当的标点符号，将口语化的停顿转换为正确标点\n4. **数字规范**：将中文数字转换为阿拉伯数字（如\"一百二十三\"→\"123\"），日期时间格式化\n5. **语气词处理**：适当保留或精简\"嗯\"、\"啊\"、\"这个\"、\"那个\"等口语填充词\n6. **分段优化**：根据语义进行合理分段\n\n注意：保持原文的语言和风格，只做纠错不做内容改写。\n\n转录文本：\n${output}".to_string(),
        },
        // 2. 深度整理 - 适合重要文档
        LLMPrompt {
            id: "deep_polish".to_string(),
            name: "deep_polish".to_string(), // i18n key: settings.postProcessing.prompts.builtin.deepPolish
            prompt: "请对以下语音转录文本进行深度整理：\n\n1. **全面纠错**：修正所有错别字、语法错误和用词不当\n2. **去口语化**：删除所有无意义的填充词（嗯、啊、那个、然后等），转换为书面语\n3. **逻辑重组**：调整语序使逻辑更清晰，合并碎片化表达\n4. **专业术语**：识别并规范专业术语、人名、地名的写法\n5. **格式规范**：添加适当的标题、列表、分段，使结构清晰\n6. **内容精炼**：删除冗余信息，保留核心内容\n\n要求：在保持原意的基础上，输出高质量的书面文本。\n\n转录文本：\n${output}".to_string(),
        },
        // 3. 会议记录整理
        LLMPrompt {
            id: "meeting_minutes".to_string(),
            name: "meeting_minutes".to_string(), // i18n key: settings.postProcessing.prompts.builtin.meetingMinutes
            prompt: "请将以下会议录音转录整理为规范的会议记录：\n\n## 整理要求：\n\n1. **信息提取**：\n   - 提取会议主题、时间、参与人（如提到）\n   - 识别会议中的决策事项和结论\n   - 记录待办事项（标注责任人和截止时间，如有）\n\n2. **内容结构化**：\n   - 使用\"会议议题\"、\"讨论要点\"、\"决策结论\"、\"待办事项\"等标题\n   - 要点使用列表形式呈现\n   - 重要内容加粗标注\n\n3. **语言优化**：\n   - 修正语音识别错误\n   - 删除口语化表达和重复内容\n   - 转换为简洁专业的书面语\n\n4. **行动项标注**：\n   - 识别并高亮显示所有行动项（TODO）\n   - 格式：【待办】事项内容（负责人）\n\n转录文本：\n${output}".to_string(),
        },
        // 4. 思路整理与写作辅助
        LLMPrompt {
            id: "idea_organization".to_string(),
            name: "idea_organization".to_string(), // i18n key: settings.postProcessing.prompts.builtin.ideaOrganization
            prompt: "请帮我整理以下语音记录中的思路和观点：\n\n## 处理要求：\n\n1. **观点提取**：\n   - 识别并提取核心观点和主要论点\n   - 区分事实陈述和个人观点\n   - 标注观点之间的逻辑关系（因果、对比、递进等）\n\n2. **结构重组**：\n   - 将零散的想法组织成有层次的结构\n   - 归类相似观点，合并重复内容\n   - 补充逻辑衔接，使思路连贯\n\n3. **关键词高亮**：\n   - 识别并高亮核心概念和关键词\n   - 标注重要的引用或数据\n\n4. **输出格式**：\n   - 使用\"核心主题\"、\"主要观点\"、\"支撑论据\"、\"待深入思考\"等结构\n   - 保留原始表达中的创意火花和个人风格\n\n5. **写作建议**（可选）：\n   - 基于整理的内容，简要建议如何展开成文章\n   - 指出可能需要补充的信息\n\n转录文本：\n${output}".to_string(),
        },
        // 5. 关键词标注模式
        LLMPrompt {
            id: "keyword_highlight".to_string(),
            name: "keyword_highlight".to_string(), // i18n key: settings.postProcessing.prompts.builtin.keywordHighlight
            prompt: "请处理以下语音转录，并特别关注关键词标注：\n\n## 处理要求：\n\n1. **基础纠错**：\n   - 修正语音识别错误和错别字\n   - 删除无意义的重复和填充词\n   - 添加适当标点\n\n2. **关键词识别与标注**：\n   - 自动识别以下类型的关键词并高亮：\n     * 重要概念和专业术语 → 【概念：xxx】\n     * 人名、地名、组织名 → 【实体：xxx】\n     * 时间、日期、数字 → 【时间/数据：xxx】\n     * 待办事项或行动点 → 【行动：xxx】\n     * 疑问或待确认内容 → 【待确认：xxx】\n\n3. **自定义关键词监听**：\n   - 如果文本中出现以下关键词，请特别标注：\n     * \"重点\"、\"重要\"、\"注意\" → 【重点】标注后续内容\n     * \"记下来\"、\"记下来\" → 【笔记】标注后续内容\n     * \"回头\"、\"之后\"、\"稍后\" → 【待办】标注后续内容\n     * \"back off\"、\"标记\"、\"highlight\" → 【高亮】标注后续内容\n\n4. **输出格式**：\n   - 先输出整理后的完整文本\n   - 然后列出提取的关键词汇总表\n\n转录文本：\n${output}".to_string(),
        },
        // 6. 访谈/采访整理
        LLMPrompt {
            id: "interview_transcript".to_string(),
            name: "interview_transcript".to_string(), // i18n key: settings.postProcessing.prompts.builtin.interviewTranscript
            prompt: "请将以下访谈录音转录整理为规范的访谈稿：\n\n## 整理要求：\n\n1. **对话格式**：\n   - 区分说话人（如能识别），使用\"问：\"\"答：\"或\"A：\"\"B：\"格式\n   - 保持对话的问答逻辑\n\n2. **内容优化**：\n   - 修正语音识别错误，特别是人名和专业术语\n   - 删除无意义的口语填充词\n   - 适当精简重复表达\n\n3. **重点标注**：\n   - 高亮显示核心观点和精彩引语\n   - 标注重要的数据、案例、结论\n   - 标记需要后续核实的内容\n\n4. **结构整理**：\n   - 根据话题转换进行分段\n   - 可在开头添加\"访谈主题\"和\"关键要点\"摘要\n\n5. **保留风格**：\n   - 保留受访者的语言风格和个性表达\n   - 不过度书面化，保持口语的生动性\n\n转录文本：\n${output}".to_string(),
        },
        // 7. 学术/技术文档整理
        LLMPrompt {
            id: "academic_document".to_string(),
            name: "academic_document".to_string(), // i18n key: settings.postProcessing.prompts.builtin.academicDocument
            prompt: "请将以下语音转录整理为规范的学术/技术文档格式：\n\n## 整理要求：\n\n1. **术语规范**：\n   - 统一专业术语的写法\n   - 标注英文术语的对应中文（或反之）\n   - 公式、代码、数据使用合适的格式标记\n\n2. **结构优化**：\n   - 使用标题层级（# ## ###）组织内容\n   - 列表、表格、代码块等使用 Markdown 格式\n   - 添加摘要或总结段落\n\n3. **引用标注**：\n   - 识别提到的文献、资料、来源\n   - 使用规范的引用格式标注\n\n4. **语言精炼**：\n   - 删除口语化表达\n   - 转换为学术/技术写作风格\n   - 保持客观、准确的表述\n\n5. **补充说明**：\n   - 标记需要进一步核实的内容\n   - 标注需要补充的图表或数据\n\n转录文本：\n${output}".to_string(),
        },
        // 8. 简洁模式 - 仅最小化修正
        LLMPrompt {
            id: "minimal_correction".to_string(),
            name: "minimal_correction".to_string(), // i18n key: settings.postProcessing.prompts.builtin.minimalCorrection
            prompt: "请对以下语音转录进行最小化修正，保持原汁原味：\n\n## 处理原则：\n\n1. **仅修正明显错误**：\n   - 修正语音识别导致的错别字（同音字错误）\n   - 修正明显的语法错误\n\n2. **保留所有内容**：\n   - 保留所有语气词（嗯、啊、这个等）\n   - 保留重复和口头禅\n   - 保留不完整的句子和停顿\n   - 保留个人语言风格\n\n3. **最小干预**：\n   - 不做内容重组\n   - 不添加总结或解释\n   - 不过度书面化\n\n4. **仅添加必要标点**：\n   - 在明显需要断句的地方添加标点\n   - 保持口语化的节奏感\n\n目标：输出应尽可能接近原始录音的真实表达，只做必要的纠错。\n\n转录文本：\n${output}".to_string(),
        },
    ]
}

pub fn ensure_post_process_defaults(settings: &mut AppSettings) -> bool {
    let mut changed = false;

    // Ensure default providers exist
    for provider in default_post_process_providers() {
        // Use match to do a single lookup - either sync existing or add new
        match settings
            .post_process_providers
            .iter_mut()
            .find(|p| p.id == provider.id)
        {
            Some(existing) => {
                // Sync supports_structured_output field for existing providers (migration)
                if existing.supports_structured_output != provider.supports_structured_output {
                    log::debug!(
                        "Updating supports_structured_output for provider '{}' from {} to {}",
                        provider.id,
                        existing.supports_structured_output,
                        provider.supports_structured_output
                    );
                    existing.supports_structured_output = provider.supports_structured_output;
                    changed = true;
                }
            }
            None => {
                // Provider doesn't exist, add it
                settings.post_process_providers.push(provider.clone());
                changed = true;
            }
        }

        if !settings.post_process_api_keys.contains_key(&provider.id) {
            settings
                .post_process_api_keys
                .insert(provider.id.clone(), String::new());
            changed = true;
        }

        let default_model = default_model_for_provider(&provider.id);
        match settings.post_process_models.get_mut(&provider.id) {
            Some(existing) => {
                if existing.is_empty() && !default_model.is_empty() {
                    *existing = default_model.clone();
                    changed = true;
                }
            }
            None => {
                settings
                    .post_process_models
                    .insert(provider.id.clone(), default_model);
                changed = true;
            }
        }
    }

    // Ensure default prompts exist (for users upgrading from older versions)
    let default_prompts = default_post_process_prompts();
    for default_prompt in default_prompts {
        if !settings
            .post_process_prompts
            .iter()
            .any(|p| p.id == default_prompt.id)
        {
            settings.post_process_prompts.push(default_prompt);
            changed = true;
        }
    }

    changed
}

pub fn get_default_post_process_shortcut() -> String {
    #[cfg(target_os = "windows")]
    return "alt+shift+space".to_string();
    #[cfg(target_os = "macos")]
    return "command_right+shift".to_string();
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    return "ctrl+shift+space".to_string();
}
