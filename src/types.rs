use std::collections::HashMap;

use regex::Regex;

pub enum Platform {
  Browser,
  Node,
  Neutral,
}

pub enum Format {
  IIFE,
  CommonJS,
  ESModule,
}

pub enum Loader {
  JS,
  JSX,
  TS,
  TSX,
  CSS,
  JSON,
  Text,
  Base64,
  File,
  Dataurl,
  Binary,
  Default,
}

pub enum LogLevel {
  Verbose,
  Debug,
  Info,
  Warning,
  Error,
  Silent,
}

pub enum Charset {
  Ascii,
  Utf8,
}

pub enum Drop {
  Console,
  Debugger,
}

pub enum Sourcemap {
  Inline,
  Linked,
  External,
  Both,
}

pub enum LegalComments {
  None,
  Inline,
  EndOfFile,
  Linked,
  External,
}

pub enum Target {
  ESNext,
  ES5,
  ES2015,
  ES2016,
  ES2017,
  ES2018,
  ES2019,
  ES2020,
  ES2021,
  ES2022,
}

pub enum EngineName {
  EngineChrome,
  EngineEdge,
  EngineFirefo,
  EngineIE,
  EngineIOS,
  EngineNode,
  EngineOpera,
  EngineSafari,
}

pub struct Engine {
  name: EngineName,
  version: String,
}

pub enum JSXMode {
  Transform,
  Preserve,
}

pub struct CommonOptions {
  /// https://esbuild.github.io/api/#sourcemap
  sourcemap: Option<Sourcemap>,
  /// https://esbuild.github.io/api/#legal-comments
  legal_comments: Option<LegalComments>,
  /// https://esbuild.github.io/api/#source-root
  source_root: Option<String>,
  /// https://esbuild.github.io/api/#sources-content
  sources_content: Option<bool>,

  /// https://esbuild.github.io/api/#format
  format: Option<Format>,
  /// https://esbuild.github.io/api/#globalName
  global_name: String,
  /// https://esbuild.github.io/api/#target
  target: Option<Target>,
  /// https://esbuild.github.io/api/#target
  engines: Option<Vec<Engine>>,

  /// https://esbuild.github.io/api/#mangle-props
  mangle_props: Option<Regex>,
  /// https://esbuild.github.io/api/#mangle-props
  reserve_props: Option<Regex>,
  /// https://esbuild.github.io/api/#mangle-props
  mangle_quoted: Option<bool>,
  /// https://esbuild.github.io/api/#mangle-props
  mangle_cache: Option<HashMap<String, Option<String>>>,
  /// https://esbuild.github.io/api/#drop
  drop: Option<Vec<Drop>>,
  /// https://esbuild.github.io/api/#minify
  minify: bool,
  /// https://esbuild.github.io/api/#minify
  minify_whitespace: bool,
  /// https://esbuild.github.io/api/#minify
  minify_identifiers: bool,
  /// https://esbuild.github.io/api/#minify
  minify_syntax: bool,
  /// https://esbuild.github.io/api/#charset
  charset: Option<Charset>,
  /// https://esbuild.github.io/api/#tree-shaking
  tree_shaking: Option<bool>,
  /// https://esbuild.github.io/api/#ignore-annotations
  ignore_annotations: bool,

  /// https://esbuild.github.io/api/#jsx
  jsx: Option<JSXMode>,
  /// https://esbuild.github.io/api/#jsx-factory
  jsx_factory: Option<String>,
  /// https://esbuild.github.io/api/#jsx-fragment
  jsx_fragment: Option<String>,

  /// https://esbuild.github.io/api/#define
  define: Option<HashMap<String, String>>,
  /// https://esbuild.github.io/api/#pure
  pure: Option<Vec<String>>,
  /// https://esbuild.github.io/api/#keep-names
  keep_names: bool,

  /// https://esbuild.github.io/api/#color
  color: Option<bool>,
  /// https://esbuild.github.io/api/#log-level
  logLevel: Option<LogLevel>,
  /// https://esbuild.github.io/api/#log-limit
  logLimit: i32,
}

pub struct TransformOptions {
  /// https://esbuild.github.io/api/#tsconfig-raw
  tsconfig_raw: String,
  /// Documentation: https://esbuild.github.io/api/#loader
  loader: Option<Loader>,
  /// https://esbuild.github.io/api/#sourcefile
  sourcefile: String,
  /// https://esbuild.github.io/api/#banner
  banner: String,
  /// https://esbuild.github.io/api/#footer
  footer: String,
}
