terraform {
    required_providers {
        docker = {
            source = "kreuzwerker/docker"
            version = "3.0.2"
        }
    }
}

provider "docker" {
    # Use on Linuxhost = "unix:///var/run/docker.sock"
    host = "tcp://127.0.0.1:2375"
}