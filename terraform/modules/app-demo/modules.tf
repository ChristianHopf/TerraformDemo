locals {
  network_id = var.network_params.network_id
  postfix = var.network_params.postfix
  project = var.network_params.project
  shortname = "appdemo"
  hostname = "${local.shortname}-${local.postfix}"

  zone = var.zone
  entrypoint = var.zone.entrypoint
  route = local.shortname
  service_port = 80
  middlewares = ""
  scheme = var.zone.https == 1 ? "https" : "http"
  host0 = var.zone.https == 1 ? var.zone.hosts[0] : "${var.zone.hosts[0]}:${var.zone.local_port}"
  path = "/"
}

# labels for entrypoint, service, route
locals {
  labels_entrypoint = [
    { label = "traefik.http.routers.${local.route}.entrypoints", value = local.entrypoint },
    { label = "traefik.http.routers.${local.shortname}.rule", value = "${var.zone.host_rule} && PathPrefix(`${local.path}`)" }
  ]
  labels_service = [
    { label = "traefik.http.routers.${local.shortname}.service", value = "${local.shortname}@docker" },
    { label = "traefik.http.services.${local.shortname}.loadbalancer.server.port", value = local.service_port }
  ]
  labels_https = [
    { label = "traefik.http.routers.${local.route}.tls", value = "le"},
    { label = "traefik.http.routers.${local.route}.tls.certresolver", value = "le" }
  ]
  labels = concat(
    var.network_params.labels,
    var.zone.labels,
    local.labels_entrypoint,
    local.labels_service,
    var.zone.https == 1 ? local.labels_https : [],
    local.middlewares == "" ? [] : [{ label = "traefik.http.middlewares.${local.shortname}.middlewares", value = local.middlewares }],
    [{ label = "role", value = local.shortname }]
  )
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
    log_opts = var.network_params.log_opts

    networks_advanced {
        name = local.network_id
    }
    networks_advanced {
        name = var.zone.network_internal_id
    }

    dynamic "labels" {
        for_each = local.labels
        content {
            label = labels.value.label
            value = labels.value.value
        }
    }
}
