function fish_command_not_found
    set -l cmd $argv[1]
    set -l txt

    if not contains -- "$cmd" "-h" "--help" "--usage" "-?"
        if which brew_which_formula > /dev/null
            set txt (brew_which_formula --explain $cmd 2> /dev/null)
        else
            set txt (brew which-formula --explain $cmd 2> /dev/null)
        end
    end

    if test -z "$txt"
        __fish_default_command_not_found_handler $cmd
    else
        string collect $txt
    end
end

function __fish_command_not_found_handler --on-event fish_command_not_found
    fish_command_not_found $argv
end
