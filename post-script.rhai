// Copyright 2023 Salesforce, Inc. All rights reserved.
file::rename(".gitignore-template", ".gitignore");

const PROJECT_MODE = "project-mode";
const UNIFIED_PROJECT = "unified";
const IMPLEMENTATION_PROJECT = "implementation";

let project_mode = variable::get(PROJECT_MODE);
if project_mode == UNIFIED_PROJECT {
    file::delete("readme-implementation.md");
    file::rename("readme-template.md", "README.md");
    file::delete("Makefile-implementation");
    file::delete("Cargo-impl.toml");
} else if project_mode == IMPLEMENTATION_PROJECT {
    file::delete("readme-template.md");
    file::rename("readme-implementation.md", "README.md");
    file::delete("Makefile");
    file::rename("Makefile-implementation", "Makefile");
    file::delete("Cargo.toml");
    file::rename("Cargo-impl.toml", "Cargo.toml");
}
