load("@rules_pkg//:pkg.bzl", "pkg_zip")
load("@bazel_gazelle//:def.bzl", "gazelle")

# gazelle:prefix github.com/adarah/satirebot
gazelle(name = "gazelle")

gazelle(
    name = "gazelle-update-repos",
    args = [
        "-from_file=satire-bot/go.mod",
    ],
    command = "update-repos",
)

pkg_zip(
    name = "ping_pong",
    srcs = ["//satire-bot/cmd/lambda/ping_pong"],
)

pkg_zip(
    name = "pong_ping",
    srcs = ["//satire-bot/cmd/lambda/pong_ping"],
)
