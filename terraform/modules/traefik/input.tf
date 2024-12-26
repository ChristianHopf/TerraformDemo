variable "admin_email" {
  type = string
}

variable "https" {
  type = number
  default = 1
}

variable "hosted_zones" {
  type = map(object({
    name = string
    hosts = list(string)
    local_port = number
    host_rule = string
    www_rule = string
  }))
}

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
