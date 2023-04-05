#!/user/bin/env bash

# install nodejs
curl -fsSL https://deb.nodesource.com/setup_19.x | sudo -E bash - &&\
sudo apt-get install -y nodejs

#install Github Copilot CLI
sudo npm install -g @githubnext/github-copilot-cli

#authenticate with Github Copilot
github-copilot-cli auth

#install AWS CLI
# sudo apt-get install -y awscli

## Run `sudo apt-get install -y nodejs` to install Node.js 19.x and npm
## You may also need development tools to build native addons:
# sudo apt-get install gcc g++ make
## To install the Yarn package manager, run:
# curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | sudo tee /usr/share/keyrings/yarnkey.gpg >/dev/null
# echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
# sudo apt-get update && sudo apt-get install yarn