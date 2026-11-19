# Geometry 1.0.1

Payjar-native, importable geometry helper module.

```pj
import Geometry;

public class main(@self){
    println(hypot(3.0,4.0));
}
```

This is a new immutable version. The legacy `Geometry@1.0` release remains unchanged because its pinned source contains an invalid package declaration and syntax errors. Importing this version requires Payjar's package loader and `pjpm` auto-sync feature.

The module does not need `import math;` because Payjar's built-in math namespace is available by default.
