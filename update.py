import re
import sys

def modify_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Update struct definition
    struct_def = r"pub is_custom: bool,\s*// Whether this is a user-provided custom model\n}"
    replacement = r"""pub is_custom: bool,            // Whether this is a user-provided custom model
    pub downloads: Option<u64>,
    pub likes: Option<u64>,
    pub created_at: Option<String>,
}"""
    content = re.sub(struct_def, replacement, content, count=1)
    
    # Update all ModelInfo instantiations to include default values
    # Regex to find `ModelInfo { ... }`
    
    # Wait, we can just replace `is_custom: false,` or `is_custom: true,`
    # Let's find all `is_custom: false,` and `is_custom: true,` that are followed by `}`.
    
    content = re.sub(r"is_custom:\s*(true|false),(\s*)}", r"is_custom: \1,\n                downloads: None,\n                likes: None,\n                created_at: None,\2}", content)

    with open(filepath, 'w') as f:
        f.write(content)
        
modify_file("src-tauri/src/managers/model.rs")
