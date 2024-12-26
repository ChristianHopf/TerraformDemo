module "app" {
    source = "../../apps/appdemo"

    env = "local"
    project = "appdemo"
    workspace = "appdemo-local"
}