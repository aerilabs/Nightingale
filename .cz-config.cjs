module.exports = {
  // Types matching your rules
  types: [
    { value: "feat", name: "feat:     new functionality" },
    { value: "fix", name: "fix:      bug fix" },
    {
      value: "refactor",
      name: "refactor: code change without behavior change",
    },
    { value: "chore", name: "chore:    tooling, deps, config" },
    { value: "docs", name: "docs:     documentation" },
    { value: "test", name: "test:     tests only" },
    { value: "style", name: "style:    formatting-only changes" },
    { value: "ci", name: "ci:       pipelines, workflows" },
  ],

  // Scopes you want to track, can be extended per repo
  scopes: [
    { name: "cursor" },
    { name: "ui" },
    { name: "piece_table" },
    { name: "tests" },
  ],

  allowCustomScopes: true, // let you type new scopes if needed
  allowBreakingChanges: ["feat", "fix"], // only these types can have breaking changes

  // Commit message length rules
  maxHeaderLength: 120,
  minBodyLength: 20, // ensures you write meaningful body
  footerPrefix: "ISSUES CLOSED:", // optional, if you want to reference issues
};