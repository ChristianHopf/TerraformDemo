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