module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      [
        "build",
        "chore",
        "ci",
        "docs",
        "feat",
        "fix",
        "perf",
        "refactor",
        "revert",
        "test",
        "style",
        "update",
      ],
    ],

    // enforce meaningful scopes
    "scope-empty": [2, "never"],

    // allow flexible casing
    "subject-case": [0],

    // longer subject lines
    "header-max-length": [2, "always", 120],

    // force detailed commits
    "body-empty": [2, "never"],
    "body-max-line-length": [2, "always", 120],
  },
};


// feat – new functionality
// fix – bug fix
// refactor – code change without behavior change
// chore – tooling, deps, config
// docs – documentation
// test – tests only
// style – formatting-only changes
// ci – pipelines, workflows