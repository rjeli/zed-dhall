{- Need to generate a lot of users?
   Use the `generate` function from the Dhall Prelude
-}
let generate = https://prelude.dhall-lang.org/List/generate

let makeUser =
      \(user : Text) ->
        let home = "/home/${user}"

        let privateKey = "${home}/.ssh/id_ed25519"

        let publicKey = "${privateKey}.pub"

        in  { home, privateKey, publicKey }

let buildUser = \(index : Natural) -> makeUser "build${Natural/show index}"

let Config = { home : Text, privateKey : Text, publicKey : Text }

in  generate 10 Config buildUser
