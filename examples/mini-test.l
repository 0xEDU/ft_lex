%option noinput nounput
%s HEADER
%x COMMENT

DIGIT   [0-9]
ID      [a-zA-Z_][a-zA-Z0-9_]*

%%

<HEADER>{ID}      { /* handle header id */ }
{DIGIT}+          { count(); }
[a-z]+/[0-9]+     { trailing(); }

%%

static void helper(void) {
    /* user subroutine */
}