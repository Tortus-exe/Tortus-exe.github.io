#!/usr/bin/julia
using .Iterators, InteractiveUtils
vowels = r"(?<=[aeuioøœv])"
a = readline()
tone_codepoints = [0x304, 0x301, 0x00, 0x316, 0x317, 0x331]
function emplaceTone(a)
    tone = last(a)
    if(tone ∉ collect("123456")) return "invalid tone" end
    inx = parse(Int, tone)
    if(inx > 6 || inx < 0) return "invalid tone" end
    if(inx == 3) return chop(a) end
    tone_char = Char(tone_codepoints[inx])
    vowel_split = split(a, vowels, limit=2)
    return join([vowel_split[1], tone_char, chop(vowel_split[2])])
end
function translate(a)
    a |> x->replace(x,"eu" => "œ") |> x->replace(x,"eo" => "ø") |> x->return x
end
while a!="q"
    m = ((x->join(x," "))∘(x->map(emplaceTone∘translate, x))∘split)(a)
    println(m)
    clipboard(m)
    global a=readline()
end