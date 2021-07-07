# Installation instructions

## Step 1 Option 1: Create application's docker image with kaniko
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
Note that `tag` name should correspond to branch name which will be used to build image. 

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

### App configuration

When generating the cluster yaml, choose **Always** for the `imagePullPolicy`.

## Step 1 Option 2: Create application's docker image manually
Image of application is created by minikube's Docker deamon.

### Prerequisites
* A Kubernetes Cluster (tested on [Minikube](https://kubernetes.io/docs/setup/minikube/))
* [Docker](https://hub.docker.com/) installation.

### Switch Docker environment to minikube's
Change shell's Docker environment by sourcing minikube's configuration.

```shell script
eval $(minikube docker-env)
```

### Build the image

```shell script
docker build -t <repository name>:<tag> .
```
Project's dependencies are downloaded and compiled only on the first build. Every subsequent build is going to use the cached layer with deps compiled.

### App configuration

When generating the cluster yaml, choose **Never** for the `imagePullPolicy`.

## Step 2: Run application

### Prerequisites
* A Kubernetes Cluster (tested on [Minikube](https://kubernetes.io/docs/setup/minikube/))
* The published image of application (done in the previous step) or the local image

### Generate App set configuration
To create file `/k8s-config/app-set.yaml`, use:
```shell script
./k8s-scripts/generate-app-set-config.sh
```

You will be prompted for repository path info, that is: `<dockerID>/<repository name>:<tag>` usage of secret (needed for private repos. A secret from the previous step will be used then.) and `imagePullPolicy` depending what way you have chosen to build the image.
Again, you can modify and use `repo-config.in`:
```shell script
./k8s-scripts/generate-app-set-config.sh < k8s-scripts/repo-config.in
```

### Run app
To start, run:
```shell script
kubectl create -f ./k8s-config/app-services.yaml
kubectl create -f ./k8s-config/app-set.yaml # file generated in the previous step
```

#### Using app
Obtain IP address by typing:
```shell script
minikube service rust-app-server --url 
```

Use client code for communication:
```shell script
cargo run --bin client <ip_address_obtained_in_previous_step>
# e.g.
cargo run --bin client 127.0.0.1:7878
```
After running client you will be able to pass messages e.g.:
```
(op) (vertex1) (vertex2)
+    1         2         
```
Available options are:
```
+ (v1) (v2)                     # adds edge
- (v1) (v2)                     # removes edge
f (path_to_file) (batch size)   # reads above commands from file and
                                # sends them in batches 
```

You can access the result of Rust program in logs:
```shell script
kubectl describe ./k8s-config/app-set.yaml
kubectl get pods -l app=rust-app
kubectl logs -f rust-app-0
...
```
It should be similar to:
```
Process-ID: 0; Cluster size: 4
Server started! Waiting on port 7878
Worker 1 of process 0 started
Worker 0 of process 0 started
Worker 3 of process 0 started
Worker 2 of process 0 started
Received a new connection
Worker 0: Waiting for msg
Worker 0: Got msg: Add 0 99732
```

Each command or batch of commands in case of using `f` option
is sent in one TCP connection so they will be passed to the same machine.
Only commands from different TCP connections can be received by
different machines.

### Clean up 
Run:
```shell script
kubectl delete -f ./k8s-config/app-set.yaml
kubectl delete -f ./k8s-config/app-services.yaml
rm /k8s-config/app-set.yaml
```
