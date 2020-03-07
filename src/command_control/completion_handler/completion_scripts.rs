pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _signedurl() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            signedurl)
                cmd="signedurl"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            configuration)
                cmd+="__configuration"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            help)
                cmd+="__help"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            set)
                cmd+="__set"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        signedurl)
            opts=" -v -h -b -k -r -t  --verbose --help --bucket --key --region --timeout  <method>  configuration help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --bucket)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -b)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -k)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --region)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        signedurl__configuration)
            opts=" -h -V  --help --version   set completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__configuration__set)
            opts=" -h -V  --help --version  <field> <value> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        signedurl__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _signedurl -o bashdefault -o default signedurl

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c signedurl -n "__fish_use_subcommand" -s b -l bucket -d 'Bucket target for signature'
complete -c signedurl -n "__fish_use_subcommand" -s k -l key -d 'Key path target. (ie: filename)'
complete -c signedurl -n "__fish_use_subcommand" -s r -l region -d 'Region target'
complete -c signedurl -n "__fish_use_subcommand" -s t -l timeout -d 'Duration URL is invalid'
complete -c signedurl -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging'
complete -c signedurl -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_use_subcommand" -f -a "configuration" -d 'Configuration options'
complete -c signedurl -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c signedurl -n "__fish_seen_subcommand_from configuration" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from configuration" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from configuration" -f -a "set" -d 'Set configuration file value to something new'
complete -c signedurl -n "__fish_seen_subcommand_from configuration" -f -a "completions" -d 'Completion scripts for various shells'
complete -c signedurl -n "__fish_seen_subcommand_from configuration" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c signedurl -n "__fish_seen_subcommand_from set" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from set" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script'
complete -c signedurl -n "__fish_seen_subcommand_from completions" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c signedurl -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c signedurl -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef signedurl

autoload -U is-at-least

_signedurl() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-b+[Bucket target for signature]' \
'--bucket=[Bucket target for signature]' \
'-k+[Key path target. (ie: filename)]' \
'--key=[Key path target. (ie: filename)]' \
'-r+[Region target]' \
'--region=[Region target]' \
'-t+[Duration URL is invalid]' \
'--timeout=[Duration URL is invalid]' \
'-v[Enable verbose logging]' \
'--verbose[Enable verbose logging]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
':method -- The type of method being requested for signing url:_files' \
":: :_signedurl_commands" \
"*::: :->signedurl" \
&& ret=0
    case $state in
    (signedurl)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:signedurl-command-$line[2]:"
        case $line[2] in
            (configuration)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_signedurl__configuration_commands" \
"*::: :->configuration" \
&& ret=0
case $state in
    (configuration)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:signedurl-configuration-command-$line[1]:"
        case $line[1] in
            (set)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::field -- Field being targeted for change:_files' \
'::value -- Target field new value:_files' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_signedurl__configuration__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:signedurl-configuration-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_signedurl_commands] )) ||
_signedurl_commands() {
    local commands; commands=(
        "configuration:Configuration options" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'signedurl commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__bash_commands] )) ||
_signedurl__configuration__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions bash commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions_commands] )) ||
_signedurl__configuration__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script" \
"fish:Fish completion script" \
"zsh:Zsh completion script" \
"powershell:PowerShell completion script" \
"elvish:Elvish completion script" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'signedurl configuration completions commands' commands "$@"
}
(( $+functions[_signedurl__configuration_commands] )) ||
_signedurl__configuration_commands() {
    local commands; commands=(
        "set:Set configuration file value to something new" \
"completions:Completion scripts for various shells" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'signedurl configuration commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__elvish_commands] )) ||
_signedurl__configuration__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions elvish commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__fish_commands] )) ||
_signedurl__configuration__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions fish commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__help_commands] )) ||
_signedurl__configuration__completions__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions help commands' commands "$@"
}
(( $+functions[_signedurl__configuration__help_commands] )) ||
_signedurl__configuration__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration help commands' commands "$@"
}
(( $+functions[_signedurl__help_commands] )) ||
_signedurl__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl help commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__powershell_commands] )) ||
_signedurl__configuration__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions powershell commands' commands "$@"
}
(( $+functions[_signedurl__configuration__set_commands] )) ||
_signedurl__configuration__set_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration set commands' commands "$@"
}
(( $+functions[_signedurl__configuration__completions__zsh_commands] )) ||
_signedurl__configuration__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'signedurl configuration completions zsh commands' commands "$@"
}

_signedurl "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'signedurl' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'signedurl'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'signedurl' {
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Bucket target for signature')
            [CompletionResult]::new('--bucket', 'bucket', [CompletionResultType]::ParameterName, 'Bucket target for signature')
            [CompletionResult]::new('-k', 'k', [CompletionResultType]::ParameterName, 'Key path target. (ie: filename)')
            [CompletionResult]::new('--key', 'key', [CompletionResultType]::ParameterName, 'Key path target. (ie: filename)')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Region target')
            [CompletionResult]::new('--region', 'region', [CompletionResultType]::ParameterName, 'Region target')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Duration URL is invalid')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'Duration URL is invalid')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('configuration', 'configuration', [CompletionResultType]::ParameterValue, 'Configuration options')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'signedurl;configuration' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set configuration file value to something new')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'signedurl;configuration;set' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'signedurl;configuration;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;completions;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;configuration;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'signedurl;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[signedurl] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'signedurl'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'signedurl'= {
            cand -b 'Bucket target for signature'
            cand --bucket 'Bucket target for signature'
            cand -k 'Key path target. (ie: filename)'
            cand --key 'Key path target. (ie: filename)'
            cand -r 'Region target'
            cand --region 'Region target'
            cand -t 'Duration URL is invalid'
            cand --timeout 'Duration URL is invalid'
            cand -v 'Enable verbose logging'
            cand --verbose 'Enable verbose logging'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand configuration 'Configuration options'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'signedurl;configuration'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand set 'Set configuration file value to something new'
            cand completions 'Completion scripts for various shells'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'signedurl;configuration;set'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script'
            cand fish 'Fish completion script'
            cand zsh 'Zsh completion script'
            cand powershell 'PowerShell completion script'
            cand elvish 'Elvish completion script'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'signedurl;configuration;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;completions;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;configuration;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'signedurl;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    