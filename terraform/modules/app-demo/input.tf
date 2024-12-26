variable "network_params" {
    type = object({
        network_id = string
        postfix = string
        project = string
        workspace = string
        env = string
        log_opts = map(any)
        labels = list(object({label = string, value = string}))
    })
}

variable "zone" {
  type = object({
    hosts = list(string)
    host_rule = string
    https = number
    labels = list(object({ label = string, value = string }))
    entrypoint = string
    local_port = number
    name = string
    network_internal_id = string
    network_public_id = string
    network_public_name = string
    postfix = string
    project = string
  })
}
