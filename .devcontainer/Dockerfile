FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/general-cache/commandhistory/.bash_history" \
    && echo "$SNIPPET" >> "/home/vscode/.bashrc"

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install software-properties-common git -y

ARG GIT=IllusionaryFrog
RUN ln -s /general-cache/ssh/$GIT /home/vscode/.ssh
RUN ln -s /general-cache/git/$GIT/.gitconfig /home/vscode/.gitconfig
