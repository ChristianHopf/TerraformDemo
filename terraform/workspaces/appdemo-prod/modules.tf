variable "hosts" {
  type=  list(string)
  default = ["localhost"]
}

module "app" {
  source = "../../apps/appdemo"

  env = "local"
  project = "appdemo"
  workspace = "appdemo-local"
  https = 1

  hosted_zones = {
    "default" = {
      name = "default"
      hosts = var.hosts
      local_port = 0
    }
  }
}
