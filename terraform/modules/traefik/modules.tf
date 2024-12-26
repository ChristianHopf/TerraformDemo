locals {
  project = var.network_params.project
  postfix = var.network_params.postfix
  network_internal_id = var.network_params.network_id
  shortname = "traefik"
  hostname = "${local.shortname}-${local.postfix}"
  entrypoint = var.https == 1 ? "https" : "traefik"
  middlewares = ""
  scheme = var.https == 1 ? "https" : "http"
}

# port config
locals {
  ports_multihost = [{ internal = 80, external = 80 }, { internal = 443, external = 443 }]
  ports_localhost = [for z in var.hosted_zones : { internal = z.local_port, external = z.local_port  }]
  ports = var.https == 1 ? local.ports_multihost : local.ports_localhost
}

# entrypoints config
locals {
  entrypoints_multihost = [
    "--entrypoints.http.address=:80",
    "--entrypoints.https.address=:443",
    "--entrypoints.http.http.redirections.entrypoint.to=https",
    "--entrypoints.http.http.redirections.entrypoint.scheme=https",
    "--entrypoints.http.http.redirections.entrypoint.permanent=true",
  ]
  entrypoints_localhost = flatten([
    for name, z in var.hosted_zones : [
      "--entrypoints.${z.name}.address=:${z.local_port}"
    ]
  ])
  entrypoints = var.https == 1 ? local.entrypoints_multihost : local.entrypoints_localhost
}

locals {
  command = compact(concat(
    local.entrypoints,
    var.https == 1 ? [
    "--certificatesresolvers.le.acme.email=${var.admin_email}",
    "--certificatesresolvers.le.acme.storage=/certificates/acme.json",
    "--certificatesresolvers.le.acme.tlschallenge=true",
    ] : [],
    [
      "--providers=docker",
      "--providers.docker.exposedbydefault=false",
      "--providers.docker.constraints=Label(`traefik.constraint-label`, `${docker_network.public.name}`)",
      "--accesslog=true",
      "--api=true",
      "--api.debug=true",
      "--api.insecure=true",
      "--api.dashboard=true",
      "--log",
    ]
  ))
}

# labels for all public containers
locals {
  labels_all_public = [
    { label = "traefik.enable", value = "true" },
    { label = "traefik.docker.network", value = docker_network.public.name },
    { label = "traefik.constraint-label", value = docker_network.public.name }
  ]
}

# public zone properties definition
locals {
  zone = {
    for name, z in var.hosted_zones : name => {
      network_internal_id = local.network_internal_id
      network_public_id = docker_network.public.id
      network_public_name = docker_network.public.name
      postfix = local.postfix
      project = local.project
      name = name
      entrypoint = var.https == 1 ? "https" : z.name
      local_port = z.local_port
      hosts = var.https == 1 ? z.hosts : ["localhost"]
      host_rule = var.https == 1 ? z.host_rule : "Host(`localhost`)"
      www_rule = var.https == 1 ? z.www_rule : ""
      https = var.https
      labels = local.labels_all_public
    }
  }
}

# dashboard config
locals {
  enable_dashboard = contains(keys(var.hosted_zones), "traefik")

  dashboard_host_rule = !local.enable_dashboard ? "" : (
    length(var.hosted_zones["traefik"].hosts) > 1 ? 
      format("(%s)", join(" || ", formatlist("Host(`%s`)", var.hosted_zones["traefik"].hosts))) :
      "Host(`${var.hosted_zones["traefik"].hosts[0]}`)"
  )

  labels_dashboard = !local.enable_dashboard ? [] : [
    { label = "traefik.http.routers.${local.shortname}.entrypoints", value = local.zone["traefik"].entrypoint },
    { label = "traefik.http.routers.${local.shortname}.service", value = "api@internal" },
    { label = "traefik.http.routers.${local.shortname}.rule", value = local.dashboard_host_rule },
    { label = "traefik.http.routers.${local.shortname}.middlewares", value = local.middlewares },
    { label = "traefik.http.routers.${local.shortname}.tls", value = "true" },
    { label = "traefik.http.routers.${local.shortname}.tls.certresolver", value = "le" },
  ]
}

# label builder
locals {
  labels = concat(
    var.network_params.labels,
    local.enable_dashboard ? local.zone["traefik"].labels : [],
    local.enable_dashboard ? local.labels_dashboard : [],
  )
}

resource "docker_network" "public" {
  name = "${local.project}-public-${local.postfix}"
}

resource "docker_image" "traefik" {
  name = "traefik:latest"
  keep_locally = true
}

resource "docker_volume" "traefik" {
  name = "${local.project}-certificates-${local.postfix}"
}

resource "docker_container" "traefik" {
  name = local.hostname
  image = docker_image.traefik.image_id
  restart = "always"
  network_mode = "bridge"
  log_opts = var.network_params.log_opts

  networks_advanced {
    name = local.network_internal_id
  }
  networks_advanced {
    name = docker_network.public.name
  }

  volumes {
    volume_name = docker_volume.traefik.name
    container_path = "/certificates"
    read_only = false
  }

  dynamic ports {
    for_each = local.ports
    content {
      internal = ports.value.internal
      external = ports.value.external
    }
  }

  command = local.command
  
  mounts {
    source = "/var/run/docker.sock"
    target = "/var/run/docker.sock"
    type = "bind"
    read_only = true
  }
}
