
trait Verb {
    fn sub();
}

trait VerbObj: Verb {
    fn obj();
}

trait VerbDat: Verb {
    fn rcp();
}

trait PassiveVerb: Verb {

}
