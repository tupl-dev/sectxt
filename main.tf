// providers
terraform {
  required_providers {
    digitalocean = { source = "digitalocean/digitalocean" }
    local        = { source = "hashicorp/local" }
    neon         = { source = "kislerdm/neon" }
  }
}

// variables
variable "database_region" { default = "aws-ap-southeast-2" }
variable "domain" { type = string }
variable "project" { default = "sectxt" }
variable "server_region" { default = "syd1" }
variable "server_username" { default = "deployer" }

// modules
module "database" {
  region  = var.database_region
  project = var.project
  source  = "./terraform/database"
}

module "server" {
  database_url     = module.database.database_url
  project          = var.project
  region           = var.server_region
  ssh_pub_key_path = "~/.ssh/keys/id_ed25519_sectxt.pub"
  username         = var.server_username
  source           = "./terraform/server"
}

// outputs
output "database_url" {
  sensitive = true
  value     = module.database.database_url
}
output "database_url_direct" {
  sensitive = true
  value     = module.database.database_url_direct
}
output "server_ip" { value = module.server.server_ip }
output "server_ssh" { value = "ssh ${var.server_username}@${module.server.server_ip} -i ~/.ssh/keys/id_ed25519_sectxt" }

// files
resource "local_sensitive_file" "ansible_inv" {
  filename = "${path.module}/ansible/inventory.ini"
  content  = <<-EOF
    [webservers]
    ${module.server.server_ip} ansible_user=${var.server_username}
  EOF
}

resource "local_sensitive_file" "ansible_vars" {
  filename = "${path.module}/ansible/vars.yml"
  content  = <<-EOF
    database_url: "${module.database.database_url}"
    domain: "${var.domain}"
  EOF
}
