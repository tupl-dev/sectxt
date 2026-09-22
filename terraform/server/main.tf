resource "digitalocean_droplet" "this" {
  image    = "debian-13-x64"
  name     = var.project
  region   = var.region
  size     = "s-1vcpu-512mb-10gb"
  ssh_keys = [digitalocean_ssh_key.this.id]

  user_data = <<-EOF
    #cloud-config
    users:
      - name: ${var.username}
        groups: sudo
        shell: /bin/bash
        sudo: ['ALL=(ALL) NOPASSWD:ALL']
        ssh_authorized_keys:
          - ${file(var.ssh_pub_key_path)}
      - name: root
        lock_passwd: true
    ssh_pwauth: false
  EOF
}

resource "digitalocean_ssh_key" "this" {
  name       = var.project
  public_key = file(var.ssh_pub_key_path)
}
