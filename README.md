# Installation instructions

## Step 1: Create application's docker image
Image of application is created by [kaniko](https://github.com/GoogleContainerTools/kaniko/) tool.

### Prerequisites
* A Kubernetes Cluster (tested on [Minikube](https://kubernetes.io/docs/setup/minikube/))
* A [DockerHub](https://hub.docker.com/) account to push built image of application

### Create a Secret
As it states in [kaniko documentation](https://github.com/GoogleContainerTools/kaniko/blob/master/docs/tutorial.md#create-a-secret-that-holds-your-authorization-token),
the secret with authorization token is needed. Create it with:

```shell script
./k8s-scripts/create-dockerhub-secret.sh
```

You will be prompted for:
* your DockerHub `username`
* your DockerHub `password`

### Generate kaniko Pod configuration
kaniko needs to know where to push image. Use:
```shell script
./k8s-scripts/generate-kaniko-pod-config.sh
```
to produce `k8s-config/kaniko-pod.yaml`, containing information of repository.
You will be prompted for repository path info, that is: `<dockerID>/<repository name>:<tag>`.

You can also modify `k8s-scripts/repo-config.in` and pass it to script:
```shell script
./k8s-scripts/generate-kaniko-pod-config.sh < k8s-scripts/repo-config.in
```

### Run kaniko pod
To start building process, run:
```shell script
kubectl create -f ./k8s-config/kaniko-volume.yaml
kubectl create -f ./k8s-config/kaniko-volume-claim.yaml
kubectl create -f ./k8s-config/kaniko-pod.yaml # file generated in the previous step
```

To check whether the build complete, use:
```shell script
kubectl get pods
```
To show the build logs, use:
```shell script
kubectl logs -f kaniko
```

After completion, the application's docker image should be published to your repository.

### Clean up creation process
Run:
```shell script
kubectl delete -f ./k8s-config/kaniko-pod.yaml
kubectl delete -f ./k8s-config/kaniko-volume-claim.yaml
kubectl delete -f ./k8s-config/kaniko-volume.yaml
rm ./k8s-config/kaniko-pod.yaml
kubectl delete secret/regcred
```

## Step 2: Run application

### Prerequisites
* A Kubernetes Cluster (tested on [Minikube](https://kubernetes.io/docs/setup/minikube/))
* The published image of application (done in the previous step)

### Generate App pod configuration
To create file `/k8s-config/app-deployment.yaml`, use:
```shell script
./k8s-scripts/generate-app-deployment-config.sh
```

You will be prompted for repository path info, that is: `<dockerID>/<repository name>:<tag>`
and usage of secret (needed for private repos. A secret from the previous step will be used then.)
Again, you can modify and use `repo-config.in`:
```shell script
./k8s-scripts/generate-app-deployment-config.sh < k8s-scripts/repo-config.in
```

### Run app
To start, run:
```shell script
kubectl create -f ./k8s-config/app-deployment.yaml # file generated in the previous step
kubectl create -f ./k8s-config/app-service.yaml
```

#### Using app (WIP)
Obtain IP address by typing:
```shell script
minikube service rust-app-server --url
curl <IP address from the command above> 
```
You should get response:
```
Hello
```

You can access the result of Rust program in logs:
```shell script
export POD_NAME=$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\n"}}{{end}}')
kubectl logs $POD_NAME
```
It should be:
```
Server started! Waiting on port 7878
((1, 2, 3), 0, 1)
((1, 2, 3), 1, -1)
((1, 2, 4), 1, 1)
((7, 8, 9), 0, 1)
```

### Clean up 
Run:
```shell script
kubectl delete -f ./k8s-config/app-service.yaml
kubectl delete -f ./k8s-config/app-deployment.yaml
rm ./k8s-config/app-deployment.yaml
```
