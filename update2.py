import re
import sys

def modify_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    content = re.sub(r"created_at:\s*None,", r"created_at: None,\n                trending_score: None,", content)

    with open(filepath, 'w') as f:
        f.write(content)
        
modify_file("src-tauri/src/managers/model.rs")
