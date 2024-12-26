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
}