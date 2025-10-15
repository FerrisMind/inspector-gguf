# Implementation Plan

- [x] 1. Create module structure and setup





  - Create the `src/gui/` directory structure with all necessary module files
  - Set up the main `mod.rs` file with proper module declarations
  - Update `src/lib.rs` to reference the new gui module structure
  - _Requirements: 1.3, 1.4, 7.5_

- [x] 2. Extract and implement theme system module





  - [x] 2.1 Create `src/gui/theme.rs` with color constants and theme functions


    - Move all color constants (INSPECTOR_BLUE, GADGET_YELLOW, etc.) to theme module
    - Implement `load_custom_font()` function in theme module
    - Implement `apply_inspector_theme()` function in theme module
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [x] 2.2 Create `src/gui/layout.rs` with adaptive sizing functions


    - Move `get_sidebar_width()`, `get_adaptive_font_size()`, and `get_adaptive_button_width()` functions
    - Ensure all adaptive layout calculations are properly modularized
    - _Requirements: 2.2, 2.5_

- [x] 3. Extract and implement export system module







  - [x] 3.1 Create `src/gui/export.rs` with all export functionality




    - Move all export functions (CSV, YAML, Markdown, HTML, PDF)
    - Move utility functions (`ensure_extension`, `sanitize_for_markdown`, `escape_markdown_text`)
    - Move `show_base64_dialog()` function


    - _Requirements: 3.1, 3.2, 3.4_

  - [x] 3.2 Ensure proper error handling and file management in export module





    - Verify all export functions maintain existing error handling
    - Test file extension management and path utilities
    - _Requirements: 3.4_

- [x] 4. Extract and implement file loader module



  - [x] 4.1 Create `src/gui/loader.rs` with loading functionality


    - Move `LoadingResult` type alias and `MetadataEntry` struct
    - Move `load_gguf_metadata_async()` function with all its logic
    - Ensure proper thread management and progress tracking
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [x] 5. Extract and implement update checker module



  - [x] 5.1 Create `src/gui/updater.rs` with update checking functionality


    - Move `check_for_updates()` function
    - Move update-related constants (CURRENT_VERSION, GITHUB_REPO)
    - Ensure proper error handling for network operations
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 6. Extract and implement panel management modules





  - [x] 6.1 Create `src/gui/panels/mod.rs` and panel structure


    - Set up panel module structure with proper re-exports
    - _Requirements: 5.1, 5.4_

  - [x] 6.2 Create `src/gui/panels/sidebar.rs` with sidebar functionality


    - Extract left sidebar rendering logic from main update() method
    - Include all button rendering and export functionality integration
    - Maintain adaptive sizing and localization support
    - _Requirements: 5.1, 5.3_

  - [x] 6.3 Create `src/gui/panels/content.rs` with central panel functionality


    - Extract central panel rendering logic including metadata display
    - Include filter functionality and drag-and-drop support
    - Maintain progress bar and loading state display
    - _Requirements: 5.1, 5.3_

  - [x] 6.4 Create `src/gui/panels/dialogs.rs` with dialog functionality


    - Extract settings dialog rendering logic
    - Extract about dialog rendering logic
    - Extract right-side panels (chat template, tokens, merges)
    - _Requirements: 5.1, 5.2, 5.3_

- [x] 7. Implement main application orchestrator





  - [x] 7.1 Create `src/gui/app.rs` with main GgufApp implementation


    - Move `GgufApp` struct definition to app module
    - Implement `Default` trait for `GgufApp`
    - Create clean `eframe::App::update()` method that coordinates between modules
    - _Requirements: 7.1, 7.2, 7.4_



  - [[x] 7.2 Implement LanguageProvider trait in app module
    - Move `LanguageProvider` implementation to app module
    - Ensure localization functionality remains intact
    - _Requirements: 7.4_

- [x] 8. Update imports and finalize module integration





  - [x] 8.1 Update `src/gui/mod.rs` with proper re-exports


    - Add all necessary public re-exports for clean API
    - Ensure external modules can access required functionality
    - _Requirements: 1.3, 7.5_

  - [x] 8.2 Update main.rs and other files to use new module structure


    - Update import statements in files that use GUI functionality
    - Ensure all external dependencies are properly maintained
    - _Requirements: 1.4, 7.4_

- [x] 9. Verification and cleanup





  - [x] 9.1 Verify all functionality works correctly after refactoring


    - Test file loading, export functionality, and UI interactions
    - Verify theme application and adaptive layout work properly
    - Test update checking and dialog functionality
    - _Requirements: 1.2, 7.4_

  - [x] 9.2 Remove original gui.rs file and clean up


    - Delete the original large `src/gui.rs` file
    - Ensure no broken imports or missing functionality
    - _Requirements: 1.1_

- [ ]* 9.3 Add module-level documentation and tests
    - Add comprehensive documentation for each module
    - Create unit tests for individual module functions
    - _Requirements: 1.3_