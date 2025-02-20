/**
 * @file A DSL for writing choose-your-own-adventure games
 * @author Ben C. Forsberg <benfrsbrg@gmail.com>
 * @license All rights reserved
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "adventure",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => seq(
      $.game,
      $.chapters,
    ),
    game: $ => seq(
      "GAME",
      $.character,
      $.locations,
    ),
    character: $ => seq(
      "CHARACTER",
      $.identifier,
    ),
    locations: $ => seq(
      "LOCATIONS",
      repeat($.location),
    ),
    location: $ => seq(
      $.identifier,
    ),
    identifier: $ => /[a-zA-Z_]\w*/,
    chapters: $ => seq(
      "CHAPTERS",
      repeat($.chapter),
    ),
    chapter: $ => seq(
      "CHAPTER",
      $.identifier,
    ),
  },
});
