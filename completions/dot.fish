function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c dot -n '__fish_using_command dot' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot' -f -a 'check'
complete -c dot -n '__fish_using_command dot' -f -a 'link'
complete -c dot -n '__fish_using_command dot' -f -a 'clean'
complete -c dot -n '__fish_using_command dot' -f -a 'root'
complete -c dot -n '__fish_using_command dot' -f -a 'clone'
complete -c dot -n '__fish_using_command dot' -f -a 'help'
complete -c dot -n '__fish_using_command dot check' -s v -l verbose -d 'Use verbose output'
complete -c dot -n '__fish_using_command dot check' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot check' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot link' -s n -l dry-run -d 'do not actually perform I/O operations'
complete -c dot -n '__fish_using_command dot link' -s v -l verbose -d 'Use verbose output'
complete -c dot -n '__fish_using_command dot link' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot link' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot clean' -s n -l dry-run -d 'do not actually perform I/O operations'
complete -c dot -n '__fish_using_command dot clean' -s v -l verbose -d 'Use verbose output'
complete -c dot -n '__fish_using_command dot clean' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot clean' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot root' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot root' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot clone' -s n -l dry-run -d 'do not actually perform I/O operations'
complete -c dot -n '__fish_using_command dot clone' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot clone' -s V -l version -d 'Prints version information'
complete -c dot -n '__fish_using_command dot help' -s h -l help -d 'Prints help information'
complete -c dot -n '__fish_using_command dot help' -s V -l version -d 'Prints version information'
