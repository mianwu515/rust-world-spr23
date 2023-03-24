# Copilot is sooooooooo cool! It helps me build a containerized actix microservice!!

# [Continuous Delivery of Rust Actix to ECR and AWS App Runner](https://nogibjj.github.io/rust-tutorial/chapter_3.html#continuous-delivery-of-rust-actix-to-ecr-and-aws-app-runner)

## How? -> 3 steps
* 0. Start with a rust cli 
[program](https://github.com/mianwu515/rust-world-spr23/tree/main/week1).  
* 1. -> to make it locally deployed
```bash
make format
make lint
make run
```
After this, you should be able to launch your localhost web service! go to http://localhost:8080/rock
* 2. To containerize your app and launch a docker container
    * 2.1 Pull a ready docker image and launch it (make sure docker is properly configured and started)
    run `make pullimageandruncontainer`

    To push an image to DockerHub, you could follow these steps:
    * `docker login` and enter your docker "user_name" and "user_secret" as prompted
    * `docker build -t [NAME OF YOUR APP] .` 
        * You could just run `make build` for this program
        * # This is to build an image out of the Dockerfile
    * `docker tag [NAME OF YOUR APP]:latest [YOUR DOCKER USER NAME]/[NAME OF YOUR APP]:latest`
    * `docker push [YOUR DOCKER USER NAME]/[NAME OF YOUR APP]:latest`
    
    * 2.2 Build and run your own docker container
        * run `make build`
        * run `make runlocallybuiltdockercontainer`

After these steps (either locally deployed or containerized), you should be able to find the website works: http://localhost:8080/paper this work
* 3. To deploy a Continuous Delivery of the web service using AWS ECR and App Runner

1. Set up AWS Cloud9 Rust env with [rustup](https://rustup.rs/), git config global settings, etc.
2. [Generate an SSH public key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent) and configure it to your github account (if you want to later develop on your cloud env using push/pull stuffs)
3. Clone the repo and `make format` and `make lint` to make sure everything works fine.
4. Go and create an ECR (Elastic Container Registry) 
![image-ECR](pictures/ECR.png)
5. Run the 4 commands in your Cloud9 env to create a docker image for your ECR, commands provided under the "View push commands" after selecting your ECR entry.
    ```bash
    aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin [aws-account-id].dkr.ecr.us-east-1.amazonaws.com # login
    docker build -t [your ECR name] . # build docker image
    docker tag [ecr-name]:latest [aws-accout-id].dkr.ecr.us-east-1.amazonaws.com/[ecr-name]:latest # tag the image
    docker push [aws-account-id].dkr.ecr.us-east-1.amazonaws.com/actix:latest # push the image to your resporitory
    ```
6. Go and create an App Runner service by selecting the created container image url
![image-app-runner](pictures/apprunner1.png)
![image-app-runner1](pictures/apprunner.png)
![image-app-runner](pictures/apprunner2.png)
remember to set the port so that it will not have conflict with other services
![image-app-runner](pictures/ecr_port.png)
7. Wait for completion of the deployment.
8. Open up the app url! 
	- Mine is https://wnqfmhu9xj.us-east-1.awsapprunner.com/rock
	- try substitute "paper", "scissors", "hello", [you name] for 
"rock"

## Note
The dev env is AWS Cloud9. If you directly run the codes from this Codespaces, it might not work. But it should work as long as you have your docker and rust env set up in your place!

## References
* [continuous-delivery-of-rust-actix-to-ECR-and-App-runner](https://nogibjj.github.io/rust-tutorial/chapter_3.html#continuous-delivery-of-rust-actix-to-ecr-and-aws-app-runner)
* [containerized-actix-continuous-delivery-to-aws-apprunner](https://github.com/nogibjj/rust-mlops-template/blob/main/README.md#containerized-actix-continuous-delivery-to-aws-app-runner)
* [webdocker-github-code-example](https://github.com/nogibjj/rust-mlops-template/tree/main/webdocker)
* [install-docker-on-debian](https://www.fosslinux.com/49959/install-docker-on-debian.html)
* [create-docker-image-pull/push-image](https://www.pluralsight.com/guides/create-docker-images-docker-hub)
* [install-and-use-docker-on-ubuntu](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04)
* [error-the-repo-does-not-have-a-release-while-installing-docker-on-ubuntu](https://stackoverflow.com/questions/73175063/the-repository-does-not-have-a-release-file-error-on-debian-11-bullseye-even)