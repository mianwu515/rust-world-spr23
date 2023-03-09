# Random-fact-generator - A Serverless Web Application

```bash
serverless deploy
```

> Deploying random-fact-generator to stage dev (us-east-1)

> ✔ Service deployed to stack random-fact-generator-dev (73s)

> endpoint: GET - https://mal9wcp1pa.execute-api.us-east-1.amazonaws.com/dev/fact
> functions:
>   rand-fact-generator-rust: random-fact-generator-dev-rand-fact-generator-rust (2.6 MB)


A detailed guide and references can be found [here](https://github.com/nogibjj/rust-on-aws-lambda)

## A handful of possible usages of AWS Lambda
* Serverless Web Application: You can use Lambda to build a serverless web application that interacts with a database and serves web pages or API endpoints. Lambda can be used to process user requests, retrieve data from the database, and return dynamic content to the user.

* Data Processing: Lambda can be used to process data in real-time or batch mode. For example, you can use Lambda to process incoming data from IoT devices or data streams and trigger downstream processes.

* Notification Services: You can use Lambda to build a notification service that sends messages via email, SMS, or push notifications. Lambda can be triggered by events such as user sign-up or purchase and then send notifications to users or administrators.

* Chatbot: You can use Lambda to build a chatbot that can interact with users and perform various tasks such as answering questions, scheduling appointments, or placing orders. Lambda can be used to process user inputs and integrate with third-party services to provide relevant responses.

* File Processing: Lambda can be used to process files such as images, videos, or documents. For example, you can use Lambda to resize images or extract text from PDF documents.

* Machine Learning: Lambda can be used to build machine learning models that can process data in real-time or batch mode. For example, you can use Lambda to train a machine learning model based on data received from IoT devices or to process data from an online marketplace to generate personalized recommendations.