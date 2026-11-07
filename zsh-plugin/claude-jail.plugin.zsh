0=${(%):-%N}
typeset -g CLAUDE_JAIL_ROOT=${0:A:h}
fpath+=($CLAUDE_JAIL_ROOT/functions)
autoload -Uz claude-jail is-at-least
: ${CLAUDE_JAIL_DEFAULT_LEVEL:=2}
