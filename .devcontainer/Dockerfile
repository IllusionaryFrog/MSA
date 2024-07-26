FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

ARG SSH=frog

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/general-cache/commandhistory/.bash_history" \
    && echo "$SNIPPET" >> "/home/vscode/.bashrc" \
    && ln -s /general-cache/ssh/$SSH /home/vscode/.ssh
