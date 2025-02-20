#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 19
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 7
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  anon_sym_GAME = 1,
  anon_sym_CHARACTER = 2,
  anon_sym_LOCATIONS = 3,
  sym_identifier = 4,
  anon_sym_CHAPTERS = 5,
  anon_sym_CHAPTER = 6,
  sym_source_file = 7,
  sym_game = 8,
  sym_character = 9,
  sym_locations = 10,
  sym_location = 11,
  sym_chapters = 12,
  sym_chapter = 13,
  aux_sym_locations_repeat1 = 14,
  aux_sym_chapters_repeat1 = 15,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_GAME] = "GAME",
  [anon_sym_CHARACTER] = "CHARACTER",
  [anon_sym_LOCATIONS] = "LOCATIONS",
  [sym_identifier] = "identifier",
  [anon_sym_CHAPTERS] = "CHAPTERS",
  [anon_sym_CHAPTER] = "CHAPTER",
  [sym_source_file] = "source_file",
  [sym_game] = "game",
  [sym_character] = "character",
  [sym_locations] = "locations",
  [sym_location] = "location",
  [sym_chapters] = "chapters",
  [sym_chapter] = "chapter",
  [aux_sym_locations_repeat1] = "locations_repeat1",
  [aux_sym_chapters_repeat1] = "chapters_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_GAME] = anon_sym_GAME,
  [anon_sym_CHARACTER] = anon_sym_CHARACTER,
  [anon_sym_LOCATIONS] = anon_sym_LOCATIONS,
  [sym_identifier] = sym_identifier,
  [anon_sym_CHAPTERS] = anon_sym_CHAPTERS,
  [anon_sym_CHAPTER] = anon_sym_CHAPTER,
  [sym_source_file] = sym_source_file,
  [sym_game] = sym_game,
  [sym_character] = sym_character,
  [sym_locations] = sym_locations,
  [sym_location] = sym_location,
  [sym_chapters] = sym_chapters,
  [sym_chapter] = sym_chapter,
  [aux_sym_locations_repeat1] = aux_sym_locations_repeat1,
  [aux_sym_chapters_repeat1] = aux_sym_chapters_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_GAME] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CHARACTER] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LOCATIONS] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_CHAPTERS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CHAPTER] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_game] = {
    .visible = true,
    .named = true,
  },
  [sym_character] = {
    .visible = true,
    .named = true,
  },
  [sym_locations] = {
    .visible = true,
    .named = true,
  },
  [sym_location] = {
    .visible = true,
    .named = true,
  },
  [sym_chapters] = {
    .visible = true,
    .named = true,
  },
  [sym_chapter] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_locations_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_chapters_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(33);
      if (lookahead == 'C') ADVANCE(14);
      if (lookahead == 'G') ADVANCE(1);
      if (lookahead == 'L') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'A') ADVANCE(17);
      END_STATE();
    case 2:
      if (lookahead == 'A') ADVANCE(21);
      END_STATE();
    case 3:
      if (lookahead == 'A') ADVANCE(22);
      END_STATE();
    case 4:
      if (lookahead == 'A') ADVANCE(9);
      END_STATE();
    case 5:
      if (lookahead == 'A') ADVANCE(28);
      END_STATE();
    case 6:
      if (lookahead == 'C') ADVANCE(39);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 7:
      if (lookahead == 'C') ADVANCE(5);
      END_STATE();
    case 8:
      if (lookahead == 'C') ADVANCE(15);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(8);
      END_STATE();
    case 9:
      if (lookahead == 'C') ADVANCE(30);
      END_STATE();
    case 10:
      if (lookahead == 'E') ADVANCE(34);
      END_STATE();
    case 11:
      if (lookahead == 'E') ADVANCE(23);
      END_STATE();
    case 12:
      if (lookahead == 'E') ADVANCE(24);
      END_STATE();
    case 13:
      if (lookahead == 'E') ADVANCE(25);
      END_STATE();
    case 14:
      if (lookahead == 'H') ADVANCE(2);
      END_STATE();
    case 15:
      if (lookahead == 'H') ADVANCE(3);
      END_STATE();
    case 16:
      if (lookahead == 'I') ADVANCE(20);
      END_STATE();
    case 17:
      if (lookahead == 'M') ADVANCE(10);
      END_STATE();
    case 18:
      if (lookahead == 'N') ADVANCE(26);
      END_STATE();
    case 19:
      if (lookahead == 'O') ADVANCE(7);
      END_STATE();
    case 20:
      if (lookahead == 'O') ADVANCE(18);
      END_STATE();
    case 21:
      if (lookahead == 'P') ADVANCE(29);
      if (lookahead == 'R') ADVANCE(4);
      END_STATE();
    case 22:
      if (lookahead == 'P') ADVANCE(31);
      END_STATE();
    case 23:
      if (lookahead == 'R') ADVANCE(47);
      END_STATE();
    case 24:
      if (lookahead == 'R') ADVANCE(35);
      END_STATE();
    case 25:
      if (lookahead == 'R') ADVANCE(27);
      END_STATE();
    case 26:
      if (lookahead == 'S') ADVANCE(36);
      END_STATE();
    case 27:
      if (lookahead == 'S') ADVANCE(45);
      END_STATE();
    case 28:
      if (lookahead == 'T') ADVANCE(16);
      END_STATE();
    case 29:
      if (lookahead == 'T') ADVANCE(11);
      END_STATE();
    case 30:
      if (lookahead == 'T') ADVANCE(12);
      END_STATE();
    case 31:
      if (lookahead == 'T') ADVANCE(13);
      END_STATE();
    case 32:
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(32);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_GAME);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_CHARACTER);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_LOCATIONS);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'A') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'E') ADVANCE(41);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'H') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'P') ADVANCE(43);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'R') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'S') ADVANCE(46);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'T') ADVANCE(38);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_CHAPTERS);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_CHAPTERS);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(44);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_CHAPTER);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 6},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 6},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 6},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 8},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 6},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 32},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 8},
  [18] = {.lex_state = 32},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_GAME] = ACTIONS(1),
    [anon_sym_CHARACTER] = ACTIONS(1),
    [anon_sym_LOCATIONS] = ACTIONS(1),
    [anon_sym_CHAPTER] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(13),
    [sym_game] = STATE(9),
    [anon_sym_GAME] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(5), 1,
      ts_builtin_sym_end,
    ACTIONS(7), 1,
      anon_sym_CHAPTER,
    STATE(4), 2,
      sym_chapter,
      aux_sym_chapters_repeat1,
  [11] = 3,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_CHAPTERS,
    STATE(5), 2,
      sym_location,
      aux_sym_locations_repeat1,
  [22] = 3,
    ACTIONS(7), 1,
      anon_sym_CHAPTER,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
    STATE(6), 2,
      sym_chapter,
      aux_sym_chapters_repeat1,
  [33] = 3,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(15), 1,
      anon_sym_CHAPTERS,
    STATE(7), 2,
      sym_location,
      aux_sym_locations_repeat1,
  [44] = 3,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    ACTIONS(19), 1,
      anon_sym_CHAPTER,
    STATE(6), 2,
      sym_chapter,
      aux_sym_chapters_repeat1,
  [55] = 3,
    ACTIONS(22), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_CHAPTERS,
    STATE(7), 2,
      sym_location,
      aux_sym_locations_repeat1,
  [66] = 2,
    ACTIONS(27), 1,
      anon_sym_CHARACTER,
    STATE(10), 1,
      sym_character,
  [73] = 2,
    ACTIONS(29), 1,
      anon_sym_CHAPTERS,
    STATE(15), 1,
      sym_chapters,
  [80] = 2,
    ACTIONS(31), 1,
      anon_sym_LOCATIONS,
    STATE(17), 1,
      sym_locations,
  [87] = 1,
    ACTIONS(33), 2,
      sym_identifier,
      anon_sym_CHAPTERS,
  [92] = 1,
    ACTIONS(35), 2,
      ts_builtin_sym_end,
      anon_sym_CHAPTER,
  [97] = 1,
    ACTIONS(37), 1,
      ts_builtin_sym_end,
  [101] = 1,
    ACTIONS(39), 1,
      sym_identifier,
  [105] = 1,
    ACTIONS(41), 1,
      ts_builtin_sym_end,
  [109] = 1,
    ACTIONS(43), 1,
      anon_sym_LOCATIONS,
  [113] = 1,
    ACTIONS(45), 1,
      anon_sym_CHAPTERS,
  [117] = 1,
    ACTIONS(47), 1,
      sym_identifier,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 11,
  [SMALL_STATE(4)] = 22,
  [SMALL_STATE(5)] = 33,
  [SMALL_STATE(6)] = 44,
  [SMALL_STATE(7)] = 55,
  [SMALL_STATE(8)] = 66,
  [SMALL_STATE(9)] = 73,
  [SMALL_STATE(10)] = 80,
  [SMALL_STATE(11)] = 87,
  [SMALL_STATE(12)] = 92,
  [SMALL_STATE(13)] = 97,
  [SMALL_STATE(14)] = 101,
  [SMALL_STATE(15)] = 105,
  [SMALL_STATE(16)] = 109,
  [SMALL_STATE(17)] = 113,
  [SMALL_STATE(18)] = 117,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chapters, 1, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [11] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_locations, 1, 0, 0),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chapters, 2, 0, 0),
  [15] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_locations, 2, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_chapters_repeat1, 2, 0, 0),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_chapters_repeat1, 2, 0, 0), SHIFT_REPEAT(18),
  [22] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_locations_repeat1, 2, 0, 0), SHIFT_REPEAT(11),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_locations_repeat1, 2, 0, 0),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_location, 1, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_chapter, 2, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 2, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_character, 2, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_game, 3, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_adventure(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
