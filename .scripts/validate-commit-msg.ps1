param(
    [string]$CommitMsgFile
)

$msg = Get-Content $CommitMsgFile -Raw

if ($msg -notmatch '^(feat|fix|docs|style|refactor|test|chore|build|ci|perf|revert)(\([a-z0-9_-]+\))?: .+$') {
    Write-Host "❌ Commit message must follow Conventional Commits format"
    Write-Host "Example: feat(editor): add cursor movement"
    exit 1
}