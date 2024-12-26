resource "random_string" "postfix" {
    length  = 8
    upper   = false
    special = false
}

module "network" {
    source = "../../modules/network"
    
    postfix = random_string.postfix.result
    project = var.project
    workspace = var.workspace
    env = var.env
    labels = var.labels
}

module "app_demo" {
    source = "../../modules/app-demo"

    network_params = module.network.params
    zone = module.traefik.zone["default"]
}

module "traefik" {
  source = "../../modules/traefik"

  network_params = module.network.params
  https = var.https

  admin_email = var.admin_email
  hosted_zones = {
    for name, z in var.hosted_zones : name => {
      name = z.name
      hosts = z.hosts
      local_port = z.local_port
      host_rule = length(z.hosts) > 1 ? format("(%s)", join(" || ", formatlist("Host(`%s`)", z.hosts))) : format("Host(`%s`)", z.hosts[0])
      www_rule = var.https == 1 ? format("(%s)", join(" || ", formatlist("Host(`www.%s`)", z.hosts))) : ""
  }
  }
}
