locals {
    network_id = var.network_params.network_id
    postfix = var.network_params.postfix
    project = var.network_params.project
    shortname = "appdemo"
    hostname = "${local.shortname}-${local.postfix}"
    labels = []
}

resource "docker_image" "app_demo" {
    name = "app-demo"
    keep_locally = true
}

resource "docker_container" "app_demo" {
    image = docker_image.app_demo.image_id
    name = local.hostname
    restart = "always"
    network_mode = "bridge"
    # log_opts = var.network_params.log_opts

    networks_advanced {
        name = local.network_id
    }

    dynamic "labels" {
        for_each = local.labels
        content {
            label = labels.value.label
            value = labels.value.value
        }
    }
}