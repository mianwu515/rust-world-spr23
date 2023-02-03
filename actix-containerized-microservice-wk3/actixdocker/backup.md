
* If you haven't installed docker, run this command or similar based on your machine type.

```bash
curl -sSL https://get.docker.com/ | sudo sh

# To use Docker as a non-root user (add your user to the "docker" group)

  sudo usermod -aG docker [username]
  
  # gpasswd -a $USER docker


  sudo service docker start

# To get your username:
  whoami

# I got this (in github codespaces):
  vscode

# so i add vscode into the docker group, which is "sudo usermod -aG docker vscode"
  
# Remember that you will have to log out and back in for this to take effect! 
```
More details see [here](https://stackoverflow.com/questions/30379381/docker-command-not-found-even-though-installed-with-apt-get)
